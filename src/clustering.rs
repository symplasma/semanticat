use crate::embedding::Embedding;
use crate::progress::Progress;
use color_eyre::eyre::{eyre, Result};
use hdbscan::{Hdbscan, HdbscanHyperParams};
use tracing::{debug, info, instrument};

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
#[instrument(skip(embeddings, progress))]
pub fn cluster(
    embeddings: &[Embedding],
    min_cluster_size: usize,
    min_samples: usize,
    progress: &Progress,
) -> Result<Vec<Assignment>> {
    if embeddings.len() < 2 {
        info!(
            count = embeddings.len(),
            "too few embeddings to cluster, treating all as noise"
        );
        progress.inc(embeddings.len() as u64);
        return Ok(vec![Assignment::Noise; embeddings.len()]);
    }

    let data: Vec<Vec<f64>> = embeddings
        .iter()
        .map(|embedding| embedding.0.iter().map(|&value| value as f64).collect())
        .collect();
    debug!(rows = data.len(), "built embedding matrix");

    let min_cluster_size = min_cluster_size.min(data.len()).max(1);
    let min_samples = min_samples.min(data.len()).max(1);

    progress.set_message("Clustering...");
    info!(min_cluster_size, min_samples, "running HDBSCAN");
    let labels = run_hdbscan(&data, min_cluster_size, min_samples)?;
    progress.inc(embeddings.len() as u64);

    let assignments: Vec<Assignment> = labels
        .into_iter()
        .map(|label| {
            if label < 0 {
                Assignment::Noise
            } else {
                Assignment::Cluster(ClusterId(label as usize))
            }
        })
        .collect();

    let noise_count = assignments
        .iter()
        .filter(|assignment| matches!(assignment, Assignment::Noise))
        .count();
    info!(
        total = assignments.len(),
        noise = noise_count,
        "clustering complete"
    );

    Ok(assignments)
}

/// Runs HDBSCAN clustering on `data`, returning the raw cluster labels.
fn run_hdbscan(
    data: &[Vec<f64>],
    min_cluster_size: usize,
    min_samples: usize,
) -> Result<Vec<i32>> {
    let hyper_params = HdbscanHyperParams::builder()
        .min_cluster_size(min_cluster_size)
        .min_samples(min_samples)
        .build();

    let clusterer = Hdbscan::new(data, hyper_params);
    clusterer
        .cluster()
        .map_err(|error| eyre!("HDBSCAN clustering failed: {error}"))
}
