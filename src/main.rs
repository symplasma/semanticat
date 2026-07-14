use clap::Parser;
use color_eyre::eyre::Result;
use std::io::{self, Read};
use tracing::{info, Level};

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

    /// Increase logging verbosity (-v for debug, -vv for trace).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

/// Maps a `-v` occurrence count to a tracing verbosity level.
fn verbosity_level(count: u8) -> Level {
    match count {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    }
}

/// Initializes the global tracing subscriber, writing to stderr so that
/// stdout remains reserved for the program's actual output.
fn init_tracing(level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_writer(io::stderr)
        .init();
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    init_tracing(verbosity_level(cli.verbose));

    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;
    info!(bytes = raw_input.len(), "read input from stdin");

    let lines = input::read_non_blank_lines(&raw_input);
    let embeddings = embedding::embed_lines(&lines)?;
    let assignments = clustering::cluster(&embeddings, cli.min_cluster_size, cli.min_samples)?;
    let grouped = grouping::group(lines, assignments);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    output::print(&grouped, &mut handle)?;

    Ok(())
}
