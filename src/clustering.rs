use crate::embedding::Embedding;
use color_eyre::eyre::{eyre, Result};
use hdbscan::{Hdbscan, HdbscanHyperParams};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tracing::{debug, info, instrument};

/// How long clustering may run before a progress spinner is shown.
const PROGRESS_BAR_DELAY: Duration = Duration::from_secs(2);

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
#[instrument(skip(embeddings))]
pub fn cluster(
    embeddings: &[Embedding],
    min_cluster_size: usize,
    min_samples: usize,
) -> Result<Vec<Assignment>> {
    if embeddings.len() < 2 {
        info!(
            count = embeddings.len(),
            "too few embeddings to cluster, treating all as noise"
        );
        return Ok(vec![Assignment::Noise; embeddings.len()]);
    }

    let data: Vec<Vec<f64>> = embeddings
        .iter()
        .map(|embedding| embedding.0.iter().map(|&value| value as f64).collect())
        .collect();
    debug!(rows = data.len(), "built embedding matrix");

    let min_cluster_size = min_cluster_size.min(data.len()).max(1);
    let min_samples = min_samples.min(data.len()).max(1);

    info!(min_cluster_size, min_samples, "running HDBSCAN");
    let labels = run_hdbscan_with_progress(data, min_cluster_size, min_samples)?;

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

/// Runs HDBSCAN on a background thread, displaying a progress spinner if
/// clustering takes longer than [`PROGRESS_BAR_DELAY`].
fn run_hdbscan_with_progress(
    data: Vec<Vec<f64>>,
    min_cluster_size: usize,
    min_samples: usize,
) -> Result<Vec<i32>> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let result = run_hdbscan(&data, min_cluster_size, min_samples);
        let _ = sender.send(result);
    });

    match receiver.recv_timeout(PROGRESS_BAR_DELAY) {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            let progress_bar = ProgressBar::new_spinner();
            progress_bar.set_style(
                ProgressStyle::with_template("{spinner:.cyan} {msg}")
                    .expect("progress bar template is valid"),
            );
            progress_bar.set_message("Clustering...");
            progress_bar.enable_steady_tick(Duration::from_millis(100));

            let result = receiver
                .recv()
                .map_err(|_| eyre!("clustering thread disconnected unexpectedly"))?;

            progress_bar.finish_and_clear();
            result
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(eyre!("clustering thread disconnected unexpectedly"))
        }
    }
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
