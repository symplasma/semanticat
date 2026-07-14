use crate::embedding::Embedding;
use avx_clustering::HDBSCAN;
use color_eyre::eyre::{eyre, Result};
use ndarray::Array2;

/// Identifies a cluster of semantically similar lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClusterId(pub usize);

/// The outcome of clustering a single line: either it belongs to a
/// meaningful cluster, or it was flagged as noise (an outlier that doesn't
/// fit well into any cluster).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assignment {
    Cluster(ClusterId),
    Noise,
}

/// Runs HDBSCAN over `embeddings` and returns one [`Assignment`] per input
/// embedding, in the same order.
///
/// NOTE: This assumes `HDBSCANResult` exposes a `labels: Vec<i32>` field
/// (with negative values indicating noise), which is a best-effort guess
/// pending verification against the real crate docs.
pub fn cluster(
    embeddings: &[Embedding],
    min_cluster_size: usize,
    min_samples: usize,
) -> Result<Vec<Assignment>> {
    let rows = embeddings.len();
    let cols = embeddings.first().map_or(0, |embedding| embedding.0.len());

    let flat: Vec<f64> = embeddings
        .iter()
        .flat_map(|embedding| embedding.0.iter().map(|&value| value as f64))
        .collect();

    let matrix = Array2::from_shape_vec((rows, cols), flat)
        .map_err(|error| eyre!("failed to build embedding matrix: {error}"))?;

    let result = HDBSCAN::new(min_cluster_size, min_samples)
        .fit(&matrix.view())
        .map_err(|error| eyre!("HDBSCAN clustering failed: {error}"))?;

    Ok(result
        .labels
        .into_iter()
        .map(|label| {
            if label < 0 {
                Assignment::Noise
            } else {
                Assignment::Cluster(ClusterId(label as usize))
            }
        })
        .collect())
}
