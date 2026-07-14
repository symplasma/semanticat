use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// A progress bar that can be transparently disabled.
///
/// Wraps [`indicatif::ProgressBar`] so callers don't need to branch on
/// whether progress reporting is enabled.
pub struct Progress(Option<ProgressBar>);

impl Progress {
    /// Creates a new progress bar tracking `total` units of work.
    ///
    /// If `enabled` is `false`, the returned [`Progress`] is a no-op.
    pub fn new(total: u64, enabled: bool) -> Self {
        if !enabled {
            return Self(None);
        }

        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} {msg} [{bar:40.cyan/blue}] {pos}/{len}",
            )
            .expect("progress bar template is valid")
            .progress_chars("=>-"),
        );
        bar.enable_steady_tick(Duration::from_millis(100));
        Self(Some(bar))
    }

    /// Sets the message shown alongside the progress bar.
    pub fn set_message(&self, message: &'static str) {
        if let Some(bar) = &self.0 {
            bar.set_message(message);
        }
    }

    /// Advances the progress bar by `delta` units.
    pub fn inc(&self, delta: u64) {
        if let Some(bar) = &self.0 {
            bar.inc(delta);
        }
    }

    /// Finishes and removes the progress bar, if it was shown.
    pub fn finish_and_clear(&self) {
        if let Some(bar) = &self.0 {
            bar.finish_and_clear();
        }
    }
}
