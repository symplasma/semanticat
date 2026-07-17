use crate::input::Line;
use crate::progress::Progress;
use color_eyre::eyre::{Result, eyre};
use kalosm::language::{Llama, LlamaSource, Task, TextStream};
use std::fmt;
use tracing::{debug, info, instrument};

/// System prompt instructing the model to produce a short, plain heading.
const SYSTEM_PROMPT: &str = "You write short, descriptive headings that summarize a group of \
related lines of text. Respond with only the heading itself: no punctuation, no quotes, and no \
explanation.";

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
#[instrument]
async fn load_model(choice: &HeadingModelChoice) -> Result<Llama> {
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
    let runtime =
        tokio::runtime::Runtime::new().map_err(|error| eyre!("failed to start async runtime: {error}"))?;

    runtime.block_on(async {
        progress.set_message("Loading heading model...");
        let model = load_model(&model_choice).await?;
        info!(?model_choice, "loaded heading model");

        progress.set_message("Generating headings...");
        let mut headings = Vec::with_capacity(clusters.len());
        for cluster in clusters {
            let heading = generate_heading(&model, cluster, max_words).await?;
            progress.inc(1);
            headings.push(heading);
        }

        Ok(headings)
    })
}
