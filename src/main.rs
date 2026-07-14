use clap::Parser;
use color_eyre::eyre::Result;
use std::io::{self, Read};

mod clustering;
mod embedding;
mod grouping;
mod input;
mod output;

/// Sort the lines of a file into semantic clusters.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Minimum number of lines required to form a cluster.
    #[arg(long, default_value_t = 2)]
    min_cluster_size: usize,

    /// Minimum number of samples in a neighborhood for HDBSCAN core points.
    #[arg(long, default_value_t = 2)]
    min_samples: usize,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;

    let lines = input::read_non_blank_lines(&raw_input);
    let embeddings = embedding::embed_lines(&lines)?;
    let assignments = clustering::cluster(&embeddings, cli.min_cluster_size, cli.min_samples)?;
    let grouped = grouping::group(lines, assignments);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    output::print(&grouped, &mut handle)?;

    Ok(())
}
