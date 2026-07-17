use crate::input::Line;
use crate::progress::Progress;
use color_eyre::eyre::{Result, eyre};
use directories::ProjectDirs;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use regex::Regex;
use std::path::PathBuf;
use std::sync::LazyLock;
use tracing::{debug, info, instrument};

/// Number of lines embedded per batch, used to report incremental progress.
const BATCH_SIZE: usize = 32;

/// Name of the cache subdirectory used to store downloaded `fastembed`
/// model files, kept separate from any other cache files this program may
/// store in the future.
const FASTEMBED_CACHE_SUBDIR: &str = "fastembed";

/// Matches an inline markdown link like `[title](url)`, capturing the link
/// title in the first group so the URL can be discarded.
static MARKDOWN_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\([^)]+\)").expect("markdown link regex is valid")
});

/// Returns `true` if `text` contains at least one markdown-style inline
/// link, e.g. `[title](url)`.
fn contains_markdown_link(text: &str) -> bool {
    MARKDOWN_LINK.is_match(text)
}

/// Replaces markdown-style inline links in `text` with just their title,
/// discarding the URL. Any other text is left untouched. If `text` contains
/// no markdown links, it is returned unchanged.
fn strip_markdown_link_urls(text: &str) -> String {
    if !contains_markdown_link(text) {
        return text.to_string();
    }
    MARKDOWN_LINK.replace_all(text, "$1").into_owned()
}

/// Computes the platform-appropriate cache directory in which downloaded
/// `fastembed` model files should be stored.
///
/// This respects platform conventions (e.g. `$XDG_CACHE_HOME` on Linux,
/// `~/Library/Caches` on macOS, `%LOCALAPPDATA%` on Windows), and stores
/// files under a `fastembed` subdirectory of this program's cache dir to
/// avoid conflicts with any other cache files.
fn fastembed_cache_dir() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "", "semanticat")
        .ok_or_else(|| eyre!("could not determine a cache directory for this platform"))?;
    Ok(project_dirs.cache_dir().join(FASTEMBED_CACHE_SUBDIR))
}

/// A dense vector representation of a single line of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding(pub Vec<f32>);

/// Computes an [`Embedding`] for each of the given `lines`, in order.
///
/// Uses the given `fastembed` text embedding `model`, downloading it on
/// first use if it is not already cached locally. Reports progress on
/// `progress` after each batch of lines is embedded.
#[instrument(skip(lines, progress))]
pub fn embed_lines(
    lines: &[Line],
    model: EmbeddingModel,
    progress: &Progress,
) -> Result<Vec<Embedding>> {
    progress.set_message("Loading model...");
    let cache_dir = fastembed_cache_dir()?;
    info!(?model, ?cache_dir, "loading fastembed model");
    let mut model = progress
        .suspend(|| {
            TextEmbedding::try_new(InitOptions::new(model).with_cache_dir(cache_dir))
        })
        .map_err(|error| eyre!("failed to load fastembed model: {error}"))?;

    progress.set_message("Embedding...");

    let texts: Vec<String> = lines
        .iter()
        .map(|line| strip_markdown_link_urls(&line.text))
        .collect();
    debug!(line_count = texts.len(), "encoding lines");

    let mut embeddings = Vec::with_capacity(texts.len());
    for batch in texts.chunks(BATCH_SIZE) {
        let vectors = model
            .embed(batch.to_vec(), None)
            .map_err(|error| eyre!("failed to compute embeddings: {error}"))?;
        progress.inc(vectors.len() as u64);
        embeddings.extend(vectors.into_iter().map(Embedding));
    }
    info!(embedding_count = embeddings.len(), "computed embeddings");

    Ok(embeddings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_unchanged() {
        assert_eq!(strip_markdown_link_urls("just some text"), "just some text");
    }

    #[test]
    fn markdown_link_keeps_title_and_drops_url() {
        let text = "check out [Rust](https://www.rust-lang.org/) today";
        assert_eq!(
            strip_markdown_link_urls(text),
            "check out Rust today"
        );
    }

    #[test]
    fn multiple_links_are_all_stripped() {
        let text = "[a](https://a.example) and [b](https://b.example)";
        assert_eq!(strip_markdown_link_urls(text), "a and b");
    }

    #[test]
    fn non_link_brackets_are_preserved() {
        let text = "array indexing uses [brackets] like this";
        assert_eq!(strip_markdown_link_urls(text), text);
    }

    #[test]
    fn detects_markdown_links() {
        assert!(contains_markdown_link("[title](url)"));
        assert!(!contains_markdown_link("no links here"));
    }

    #[test]
    fn fastembed_cache_dir_ends_with_fastembed_subdir() {
        let cache_dir = fastembed_cache_dir().expect("cache dir should be determinable");
        assert_eq!(
            cache_dir.file_name().and_then(|name| name.to_str()),
            Some(FASTEMBED_CACHE_SUBDIR)
        );
    }
}
