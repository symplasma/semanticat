use arboard::Clipboard;
use clap::Parser;
use color_eyre::eyre::Result;
use std::io::{self, IsTerminal, Read};
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

/// Reads the current contents of the system clipboard as text.
fn read_clipboard() -> Result<String> {
    let mut clipboard = Clipboard::new()?;
    let text = clipboard.get_text()?;
    Ok(text)
}

/// Reads all of stdin to a string.
fn read_stdin() -> Result<String> {
    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;
    Ok(raw_input)
}

/// Reads input from stdin if it is piped and non-empty, otherwise falls
/// back to the system clipboard.
fn read_input() -> Result<String> {
    if io::stdin().is_terminal() {
        info!("stdin is a terminal, reading from clipboard instead");
        return read_clipboard();
    }

    let raw_input = read_stdin()?;
    if raw_input.trim().is_empty() {
        info!("stdin was empty, falling back to clipboard");
        read_clipboard()
    } else {
        Ok(raw_input)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    init_tracing(verbosity_level(cli.verbose));

    let raw_input = read_input()?;
    info!(bytes = raw_input.len(), "read input");

    let lines = input::read_non_blank_lines(&raw_input);
    let embeddings = embedding::embed_lines(&lines)?;
    let assignments = clustering::cluster(&embeddings, cli.min_cluster_size, cli.min_samples)?;
    let grouped = grouping::group(lines, assignments);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    output::print(&grouped, &mut handle)?;

    Ok(())
}
