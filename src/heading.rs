use crate::input::Line;
use crate::progress::Progress;
use color_eyre::eyre::{Result, eyre};
use directories::ProjectDirs;
use gag::Gag;
use kalosm::language::{Llama, LlamaSource, Task, TextStream};
use std::fmt;
use std::path::PathBuf;
use tracing::{debug, info, instrument};

/// System prompt instructing the model to produce a short, plain heading.
const SYSTEM_PROMPT: &str = "You write short, descriptive headings that summarize a group of \
related lines of text. Respond with only the heading itself: no punctuation, no quotes, and no \
explanation.";

/// Name of the cache subdirectory used to store downloaded `kalosm` model
/// files, kept separate from any other cache files this program may store
/// in the future.
const KALOSM_CACHE_SUBDIR: &str = "kalosm";

/// Computes the platform-appropriate cache directory in which downloaded
/// `kalosm` model files should be stored, mirroring the approach used for
/// `fastembed` in [`crate::embedding`].
fn kalosm_cache_dir() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "", "semanticat")
        .ok_or_else(|| eyre!("could not determine a cache directory for this platform"))?;
    Ok(project_dirs.cache_dir().join(KALOSM_CACHE_SUBDIR))
}

/// Points `kalosm`'s underlying `hf-hub` client at our own cache directory,
/// unless the user has already configured `HF_HUB_CACHE` themselves.
///
/// `kalosm` does not expose a builder method for its cache location, so we
/// configure it via the `HF_HUB_CACHE` environment variable that `hf-hub`
/// reads on startup, per `design/kalosm-model-caching.md`.
///
/// # Safety invariant
///
/// `std::env::set_var` is `unsafe` because mutating the environment races
/// with any other thread reading it. Callers must invoke this *before*
/// spawning the Tokio runtime (or any other threads), so no concurrent
/// reads can occur.
fn configure_kalosm_cache_dir() -> Result<()> {
    if std::env::var_os("HF_HUB_CACHE").is_some() {
        debug!("HF_HUB_CACHE already set, leaving kalosm cache location unchanged");
        return Ok(());
    }

    let cache_dir = kalosm_cache_dir()?;
    info!(?cache_dir, "pointing kalosm at its own cache directory");
    // SAFETY: this is called before the Tokio runtime (or any other
    // threads) are spawned, so no other thread can be concurrently
    // reading the environment.
    unsafe {
        std::env::set_var("HF_HUB_CACHE", cache_dir);
    }
    Ok(())
}

/// Friendly names for the supported `kalosm` heading-generation models.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum HeadingModelChoice {
    /// A general-purpose chat model, prompted to summarize each cluster
    /// (default).
    Summarizer,
    /// A Llama-family chat model (Llama-3.1-8B-Instruct).
    Llama,
    /// microsoft/Phi-3-mini-4k-instruct.
    #[value(name = "phi-3")]
    Phi3,
}

/// A short, descriptive heading generated for a cluster of lines.
///
/// The constructor enforces the caller-specified maximum word count, so
/// once a `Heading` exists it is guaranteed to satisfy that constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct Heading(String);

impl Heading {
    /// Builds a [`Heading`] from raw model output, trimming surrounding
    /// whitespace/quotes and truncating to at most `max_words` words.
    pub(crate) fn new(raw: &str, max_words: usize) -> Self {
        let cleaned = raw
            .trim()
            .trim_matches(|c: char| c == '"' || c == '\'')
            .trim();
        let truncated = cleaned
            .split_whitespace()
            .take(max_words)
            .collect::<Vec<_>>()
            .join(" ");
        Self(truncated)
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Loads the `kalosm` model corresponding to `choice`.
///
/// `kalosm`'s underlying `candle`/`kalosm-common` device-selection code
/// prints a hard-coded message directly to **stdout** via `println!` when
/// it detects there is no GPU acceleration available (e.g. "Running on
/// CPU, to run on GPU, build with the cuda feature enabled..."). This was
/// confirmed empirically: the message persists under `2>/dev/null` but
/// disappears under `1>/dev/null`. Because it bypasses the `tracing`
/// framework entirely, it can't be filtered via an `EnvFilter` (see
/// `design/crate_docs/kalosm-unwanted-message.mb`, section 4, "Hard
/// Suppression"). We use the `gag` crate to temporarily redirect the
/// process's stdout (and, defensively, stderr, in case a future `kalosm`
/// version moves this message there, or other stderr diagnostics appear
/// during loading) to `/dev/null` for the duration of model loading,
/// restoring both automatically once the guards are dropped at the end of
/// this function.
///
/// Note this suppresses *all* stdout/stderr output while the guards are
/// held, not just the unwanted message. Since our own program never
/// writes its actual results to stdout until after headings are fully
/// generated (see `main`), suppressing stdout during model loading is
/// safe and won't hide any of our own output.
#[instrument]
async fn load_model(choice: &HeadingModelChoice) -> Result<Llama> {
    let _stdout_gag =
        Gag::stdout().map_err(|error| eyre!("failed to suppress model-loading stdout: {error}"))?;
    let _stderr_gag =
        Gag::stderr().map_err(|error| eyre!("failed to suppress model-loading stderr: {error}"))?;

    let model = match choice {
        HeadingModelChoice::Summarizer => Llama::new_chat().await,
        HeadingModelChoice::Llama => {
            Llama::builder()
                .with_source(LlamaSource::llama_3_1_8b_chat())
                .build()
                .await
        }
        HeadingModelChoice::Phi3 => Llama::phi_3().await,
    }
    .map_err(|error| eyre!("failed to load heading model: {error}"))?;

    Ok(model)
}

/// Joins a cluster's lines into a single prompt body.
fn cluster_prompt(lines: &[Line]) -> String {
    lines
        .iter()
        .map(|line| line.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generates a single [`Heading`] for `lines` using `model`.
async fn generate_heading(model: &Llama, lines: &[Line], max_words: usize) -> Result<Heading> {
    let task = Task::new(model.clone(), SYSTEM_PROMPT);
    let prompt = cluster_prompt(lines);

    let raw = task.run(prompt).all_text().await;
    debug!(?raw, "generated raw heading text");

    Ok(Heading::new(&raw, max_words))
}

/// Generates a [`Heading`] for each cluster in `clusters`, in order.
///
/// Loads `model_choice`'s model once and reuses it for every cluster.
/// Reports progress on `progress` as each heading completes. Spins up its
/// own single-threaded Tokio runtime, since the rest of the program is
/// synchronous.
#[instrument(skip(clusters, progress))]
pub fn generate_headings(
    clusters: &[Vec<Line>],
    model_choice: HeadingModelChoice,
    max_words: usize,
    progress: &Progress,
) -> Result<Vec<Heading>> {
    configure_kalosm_cache_dir()?;

    let runtime =
        tokio::runtime::Runtime::new().map_err(|error| eyre!("failed to start async runtime: {error}"))?;

    progress.set_message("Loading heading model...");
    let model = progress.suspend(|| runtime.block_on(load_model(&model_choice)))?;
    info!(?model_choice, "loaded heading model");

    progress.set_message("Generating headings...");
    runtime.block_on(async {
        let mut headings = Vec::with_capacity(clusters.len());
        for cluster in clusters {
            let heading = generate_heading(&model, cluster, max_words).await?;
            progress.inc(1);
            headings.push(heading);
        }

        Ok(headings)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kalosm_cache_dir_ends_with_kalosm_subdir() {
        let cache_dir = kalosm_cache_dir().expect("cache dir should be determinable");
        assert_eq!(
            cache_dir.file_name().and_then(|name| name.to_str()),
            Some(KALOSM_CACHE_SUBDIR)
        );
    }
}
