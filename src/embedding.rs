use crate::input::Line;
use color_eyre::eyre::{eyre, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::{debug, info, instrument};

/// A dense vector representation of a single line of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding(pub Vec<f32>);

/// Computes an [`Embedding`] for each of the given `lines`, in order.
///
/// Uses `fastembed`'s default text embedding model, downloading it on first
/// use if it is not already cached locally.
#[instrument(skip(lines))]
pub fn embed_lines(lines: &[Line]) -> Result<Vec<Embedding>> {
    info!("loading fastembed model");
    let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))
        .map_err(|error| eyre!("failed to load fastembed model: {error}"))?;

    let texts: Vec<&str> = lines.iter().map(|line| line.text.as_str()).collect();
    debug!(line_count = texts.len(), "encoding lines");
    let vectors = model
        .embed(texts, None)
        .map_err(|error| eyre!("failed to compute embeddings: {error}"))?;
    info!(embedding_count = vectors.len(), "computed embeddings");

    Ok(vectors.into_iter().map(Embedding).collect())
}
