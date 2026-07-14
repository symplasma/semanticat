use crate::input::Line;
use crate::progress::Progress;
use color_eyre::eyre::{Result, eyre};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::{debug, info, instrument};

/// Number of lines embedded per batch, used to report incremental progress.
const BATCH_SIZE: usize = 32;

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
    info!(?model, "loading fastembed model");
    let mut model = TextEmbedding::try_new(InitOptions::new(model))
        .map_err(|error| eyre!("failed to load fastembed model: {error}"))?;

    progress.set_message("Embedding...");

    let texts: Vec<&str> = lines.iter().map(|line| line.text.as_str()).collect();
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
