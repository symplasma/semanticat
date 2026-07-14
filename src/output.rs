use crate::grouping::Grouped;
use std::io::{self, Write};
use tracing::{debug, instrument};

/// Writes `grouped` to `writer`.
///
/// Each cluster's lines are printed in original order, followed by a blank
/// line separator. Noise lines (if any) are printed last, also separated
/// from the preceding cluster by a blank line.
#[instrument(skip(grouped, writer))]
pub fn print(grouped: &Grouped, writer: &mut impl Write) -> io::Result<()> {
    debug!(
        cluster_count = grouped.clusters.len(),
        noise_count = grouped.noise.len(),
        "writing output"
    );

    for cluster in &grouped.clusters {
        for line in cluster {
            writeln!(writer, "{}", line.text)?;
        }
        writeln!(writer)?;
    }

    for line in &grouped.noise {
        writeln!(writer, "{}", line.text)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Line;

    fn line(text: &str) -> Line {
        Line {
            text: text.to_string(),
            original_index: 0,
        }
    }

    #[test]
    fn separates_clusters_with_blank_lines() {
        let grouped = Grouped {
            clusters: vec![vec![line("a"), line("b")], vec![line("c")]],
            noise: vec![line("d")],
        };

        let mut output = Vec::new();
        print(&grouped, &mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "a\nb\n\nc\n\nd\n");
    }

    #[test]
    fn blank_line_follows_last_cluster_even_without_noise() {
        let grouped = Grouped {
            clusters: vec![vec![line("a")]],
            noise: vec![],
        };

        let mut output = Vec::new();
        print(&grouped, &mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "a\n\n");
    }
}
