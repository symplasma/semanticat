use crate::embedding::Embedding;
use color_eyre::eyre::Result;

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
/// NOTE: `avx_clustering::HDBSCAN::new(min_cluster_size, min_samples)`
/// constructs the clusterer; the `.fit(&data)` call below is a best-effort
/// guess at the method used to actually run clustering and is pending
/// verification against the real crate docs.
pub fn cluster(
    embeddings: &[Embedding],
    min_cluster_size: usize,
    min_samples: usize,
) -> Result<Vec<Assignment>> {
    let data: Vec<Vec<f32>> = embeddings
        .iter()
        .map(|embedding| embedding.0.clone())
        .collect();

    let labels = avx_clustering::HDBSCAN::new(min_cluster_size, min_samples).fit(&data);

    Ok(labels
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
