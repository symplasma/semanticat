use crate::input::Line;
use color_eyre::eyre::{eyre, Result};
use model2vec_rs::model::StaticModel;

/// A dense vector representation of a single line of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding(pub Vec<f32>);

/// Computes an [`Embedding`] for each of the given `lines`, in order.
///
/// Uses `model2vec-rs`'s bundled default static model, so no model download
/// or configuration is required.
///
/// NOTE: The exact `StaticModel` API (constructor name, arguments) is a
/// placeholder pending verification against the real crate docs.
pub fn embed_lines(lines: &[Line]) -> Result<Vec<Embedding>> {
    let model = StaticModel::from_pretrained("minishlab/potion-base-8M", None, None, None)
        .map_err(|error| eyre!("failed to load default model2vec model: {error}"))?;

    let texts: Vec<String> = lines.iter().map(|line| line.text.clone()).collect();
    let vectors = model.encode(&texts);

    Ok(vectors.into_iter().map(Embedding).collect())
}
