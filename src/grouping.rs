use crate::clustering::{Assignment, ClusterId};
use crate::input::Line;
use std::collections::HashMap;

/// Lines grouped by cluster (in first-appearance order), plus any noise
/// lines kept separately at the end.
#[derive(Debug, Clone, PartialEq)]
pub struct Grouped {
    pub clusters: Vec<Vec<Line>>,
    pub noise: Vec<Line>,
}

/// Groups `lines` according to their parallel `assignments`.
///
/// Clusters are ordered by first appearance (the order in which their first
/// member line occurs in the original input). Lines within a cluster, and
/// noise lines, retain their original relative order.
pub fn group(lines: Vec<Line>, assignments: Vec<Assignment>) -> Grouped {
    let mut cluster_order: Vec<ClusterId> = Vec::new();
    let mut cluster_lines: HashMap<ClusterId, Vec<Line>> = HashMap::new();
    let mut noise = Vec::new();

    for (line, assignment) in lines.into_iter().zip(assignments) {
        match assignment {
            Assignment::Cluster(id) => {
                cluster_lines
                    .entry(id)
                    .or_insert_with(|| {
                        cluster_order.push(id);
                        Vec::new()
                    })
                    .push(line);
            }
            Assignment::Noise => noise.push(line),
        }
    }

    let clusters = cluster_order
        .into_iter()
        .map(|id| cluster_lines.remove(&id).unwrap_or_default())
        .collect();

    Grouped { clusters, noise }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(text: &str, index: usize) -> Line {
        Line {
            text: text.to_string(),
            original_index: index,
        }
    }

    #[test]
    fn groups_by_first_appearance_order() {
        let lines = vec![line("a", 0), line("b", 1), line("c", 2), line("d", 3)];
        let assignments = vec![
            Assignment::Cluster(ClusterId(5)),
            Assignment::Cluster(ClusterId(2)),
            Assignment::Cluster(ClusterId(5)),
            Assignment::Noise,
        ];

        let grouped = group(lines, assignments);

        assert_eq!(grouped.clusters.len(), 2);
        assert_eq!(
            grouped.clusters[0]
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "c"]
        );
        assert_eq!(
            grouped.clusters[1]
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>(),
            vec!["b"]
        );
        assert_eq!(
            grouped
                .noise
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>(),
            vec!["d"]
        );
    }
}
