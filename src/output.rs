use crate::grouping::Grouped;
use crate::heading::Heading;
use std::io::{self, Write};
use tracing::{debug, instrument};

/// Writes `grouped` to `writer`.
///
/// Each cluster's lines are printed in original order, followed by a blank
/// line separator. If `headings` is provided, each cluster is preceded by
/// a `## <heading>` line, and (only in that case) a `## Ungrouped` heading
/// is printed before any noise lines, provided at least one exists. Noise
/// lines are printed last, separated from the preceding cluster by a blank
/// line.
#[instrument(skip(grouped, headings, writer))]
pub fn print(
    grouped: &Grouped,
    headings: Option<&[Heading]>,
    writer: &mut impl Write,
) -> io::Result<()> {
    debug!(
        cluster_count = grouped.clusters.len(),
        noise_count = grouped.noise.len(),
        has_headings = headings.is_some(),
        "writing output"
    );

    for (index, cluster) in grouped.clusters.iter().enumerate() {
        if let Some(heading) = headings.and_then(|headings| headings.get(index)) {
            writeln!(writer, "## {heading}")?;
        }

        for line in cluster {
            writeln!(writer, "{}", line.text)?;
        }
        writeln!(writer)?;
    }

    if headings.is_some() && !grouped.noise.is_empty() {
        writeln!(writer, "## Ungrouped")?;
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
        print(&grouped, None, &mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "a\nb\n\nc\n\nd\n");
    }

    #[test]
    fn blank_line_follows_last_cluster_even_without_noise() {
        let grouped = Grouped {
            clusters: vec![vec![line("a")]],
            noise: vec![],
        };

        let mut output = Vec::new();
        print(&grouped, None, &mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "a\n\n");
    }

    #[test]
    fn prints_headings_before_each_cluster_when_provided() {
        let grouped = Grouped {
            clusters: vec![vec![line("a")], vec![line("b")]],
            noise: vec![],
        };
        let headings = vec![Heading::new("First", 10), Heading::new("Second", 10)];

        let mut output = Vec::new();
        print(&grouped, Some(&headings), &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "## First\na\n\n## Second\nb\n\n"
        );
    }

    #[test]
    fn ungrouped_heading_only_appears_when_headings_enabled_and_noise_present() {
        let grouped = Grouped {
            clusters: vec![],
            noise: vec![line("d")],
        };

        let mut output = Vec::new();
        print(&grouped, None, &mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "d\n");

        let mut output = Vec::new();
        print(&grouped, Some(&[]), &mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "## Ungrouped\nd\n");
    }
}
