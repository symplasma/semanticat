use arboard::Clipboard;
use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use fastembed::EmbeddingModel;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use tracing::{Level, info};

mod clustering;
mod embedding;
mod grouping;
mod input;
mod output;
mod progress;

/// Friendly names for the supported `fastembed` text embedding models.
#[derive(Clone, Debug, clap::ValueEnum)]
enum ModelChoice {
    /// sentence-transformers/all-MiniLM-L6-v2 (384 dimensions).
    AllMiniLmL6V2,
    /// Quantized sentence-transformers/all-MiniLM-L6-v2.
    AllMiniLmL6V2Q,
    /// sentence-transformers/all-MiniLM-L12-v2.
    AllMiniLmL12V2,
    /// Quantized sentence-transformers/all-MiniLM-L12-v2.
    AllMiniLmL12V2Q,
    /// sentence-transformers/all-mpnet-base-v2.
    AllMpnetBaseV2,
    /// BAAI/bge-base-en-v1.5 (768 dimensions).
    BgeBase,
    /// Quantized BAAI/bge-base-en-v1.5.
    BgeBaseQ,
    /// BAAI/bge-large-en-v1.5 (1024 dimensions).
    BgeLarge,
    /// Quantized BAAI/bge-large-en-v1.5.
    BgeLargeQ,
    /// BAAI/bge-small-en-v1.5 (default).
    BgeSmall,
    /// Quantized BAAI/bge-small-en-v1.5.
    BgeSmallQ,
    /// nomic-ai/nomic-embed-text-v1.
    NomicEmbedTextV1,
    /// nomic-ai/nomic-embed-text-v1.5.
    NomicEmbedTextV15,
    /// Quantized nomic-ai/nomic-embed-text-v1.5.
    NomicEmbedTextV15Q,
    /// sentence-transformers/paraphrase-MiniLM-L6-v2.
    ParaphraseMlMiniLmL12V2,
    /// Quantized sentence-transformers/paraphrase-MiniLM-L6-v2.
    ParaphraseMlMiniLmL12V2Q,
    /// sentence-transformers/paraphrase-mpnet-base-v2.
    ParaphraseMlMpnetBaseV2,
    /// BAAI/bge-small-zh-v1.5.
    BgeSmallZh,
    /// BAAI/bge-large-zh-v1.5.
    BgeLargeZh,
    /// BAAI/bge-m3.
    BgeM3,
    /// lightonai/modernbert-embed-large.
    ModernBertEmbedLarge,
    /// intfloat/multilingual-e5-small.
    MultilingualE5Small,
    /// intfloat/multilingual-e5-base.
    MultilingualE5Base,
    /// intfloat/multilingual-e5-large.
    MultilingualE5Large,
    /// mixedbread-ai/mxbai-embed-large-v1.
    MxbaiEmbedLargeV1,
    /// Quantized mixedbread-ai/mxbai-embed-large-v1.
    MxbaiEmbedLargeV1Q,
    /// Alibaba-NLP/gte-base-en-v1.5.
    GteBase,
    /// Quantized Alibaba-NLP/gte-base-en-v1.5.
    GteBaseQ,
    /// Alibaba-NLP/gte-large-en-v1.5.
    GteLarge,
    /// Quantized Alibaba-NLP/gte-large-en-v1.5.
    GteLargeQ,
    /// Qdrant/clip-ViT-B-32-text.
    ClipVitB32,
    /// jinaai/jina-embeddings-v2-base-code.
    JinaEmbeddingsV2BaseCode,
    /// jinaai/jina-embeddings-v2-base-en.
    JinaEmbeddingsV2BaseEn,
    /// onnx-community/embeddinggemma-300m-ONNX.
    EmbeddingGemma300M,
    /// Quantized (4-bit) onnx-community/embeddinggemma-300m-ONNX.
    EmbeddingGemma300MQ4,
    /// Quantized onnx-community/embeddinggemma-300m-ONNX.
    EmbeddingGemma300MQ,
    /// snowflake/snowflake-arctic-embed-xs.
    SnowflakeArcticEmbedXs,
    /// Quantized snowflake/snowflake-arctic-embed-xs.
    SnowflakeArcticEmbedXsQ,
    /// snowflake/snowflake-arctic-embed-s.
    SnowflakeArcticEmbedS,
    /// Quantized snowflake/snowflake-arctic-embed-s.
    SnowflakeArcticEmbedSQ,
    /// snowflake/snowflake-arctic-embed-m.
    SnowflakeArcticEmbedM,
    /// Quantized snowflake/snowflake-arctic-embed-m.
    SnowflakeArcticEmbedMQ,
    /// snowflake/snowflake-arctic-embed-m-long.
    SnowflakeArcticEmbedMLong,
    /// Quantized snowflake/snowflake-arctic-embed-m-long.
    SnowflakeArcticEmbedMLongQ,
    /// snowflake/snowflake-arctic-embed-l.
    SnowflakeArcticEmbedL,
    /// Quantized snowflake/snowflake-arctic-embed-l.
    SnowflakeArcticEmbedLQ,
}

impl From<ModelChoice> for EmbeddingModel {
    fn from(choice: ModelChoice) -> Self {
        match choice {
            ModelChoice::AllMiniLmL6V2 => EmbeddingModel::AllMiniLML6V2,
            ModelChoice::AllMiniLmL6V2Q => EmbeddingModel::AllMiniLML6V2Q,
            ModelChoice::AllMiniLmL12V2 => EmbeddingModel::AllMiniLML12V2,
            ModelChoice::AllMiniLmL12V2Q => EmbeddingModel::AllMiniLML12V2Q,
            ModelChoice::AllMpnetBaseV2 => EmbeddingModel::AllMpnetBaseV2,
            ModelChoice::BgeBase => EmbeddingModel::BGEBaseENV15,
            ModelChoice::BgeBaseQ => EmbeddingModel::BGEBaseENV15Q,
            ModelChoice::BgeLarge => EmbeddingModel::BGELargeENV15,
            ModelChoice::BgeLargeQ => EmbeddingModel::BGELargeENV15Q,
            ModelChoice::BgeSmall => EmbeddingModel::BGESmallENV15,
            ModelChoice::BgeSmallQ => EmbeddingModel::BGESmallENV15Q,
            ModelChoice::NomicEmbedTextV1 => EmbeddingModel::NomicEmbedTextV1,
            ModelChoice::NomicEmbedTextV15 => EmbeddingModel::NomicEmbedTextV15,
            ModelChoice::NomicEmbedTextV15Q => EmbeddingModel::NomicEmbedTextV15Q,
            ModelChoice::ParaphraseMlMiniLmL12V2 => EmbeddingModel::ParaphraseMLMiniLML12V2,
            ModelChoice::ParaphraseMlMiniLmL12V2Q => EmbeddingModel::ParaphraseMLMiniLML12V2Q,
            ModelChoice::ParaphraseMlMpnetBaseV2 => EmbeddingModel::ParaphraseMLMpnetBaseV2,
            ModelChoice::BgeSmallZh => EmbeddingModel::BGESmallZHV15,
            ModelChoice::BgeLargeZh => EmbeddingModel::BGELargeZHV15,
            ModelChoice::BgeM3 => EmbeddingModel::BGEM3,
            ModelChoice::ModernBertEmbedLarge => EmbeddingModel::ModernBertEmbedLarge,
            ModelChoice::MultilingualE5Small => EmbeddingModel::MultilingualE5Small,
            ModelChoice::MultilingualE5Base => EmbeddingModel::MultilingualE5Base,
            ModelChoice::MultilingualE5Large => EmbeddingModel::MultilingualE5Large,
            ModelChoice::MxbaiEmbedLargeV1 => EmbeddingModel::MxbaiEmbedLargeV1,
            ModelChoice::MxbaiEmbedLargeV1Q => EmbeddingModel::MxbaiEmbedLargeV1Q,
            ModelChoice::GteBase => EmbeddingModel::GTEBaseENV15,
            ModelChoice::GteBaseQ => EmbeddingModel::GTEBaseENV15Q,
            ModelChoice::GteLarge => EmbeddingModel::GTELargeENV15,
            ModelChoice::GteLargeQ => EmbeddingModel::GTELargeENV15Q,
            ModelChoice::ClipVitB32 => EmbeddingModel::ClipVitB32,
            ModelChoice::JinaEmbeddingsV2BaseCode => EmbeddingModel::JinaEmbeddingsV2BaseCode,
            ModelChoice::JinaEmbeddingsV2BaseEn => EmbeddingModel::JinaEmbeddingsV2BaseEN,
            ModelChoice::EmbeddingGemma300M => EmbeddingModel::EmbeddingGemma300M,
            ModelChoice::EmbeddingGemma300MQ4 => EmbeddingModel::EmbeddingGemma300MQ4,
            ModelChoice::EmbeddingGemma300MQ => EmbeddingModel::EmbeddingGemma300MQ,
            ModelChoice::SnowflakeArcticEmbedXs => EmbeddingModel::SnowflakeArcticEmbedXS,
            ModelChoice::SnowflakeArcticEmbedXsQ => EmbeddingModel::SnowflakeArcticEmbedXSQ,
            ModelChoice::SnowflakeArcticEmbedS => EmbeddingModel::SnowflakeArcticEmbedS,
            ModelChoice::SnowflakeArcticEmbedSQ => EmbeddingModel::SnowflakeArcticEmbedSQ,
            ModelChoice::SnowflakeArcticEmbedM => EmbeddingModel::SnowflakeArcticEmbedM,
            ModelChoice::SnowflakeArcticEmbedMQ => EmbeddingModel::SnowflakeArcticEmbedMQ,
            ModelChoice::SnowflakeArcticEmbedMLong => EmbeddingModel::SnowflakeArcticEmbedMLong,
            ModelChoice::SnowflakeArcticEmbedMLongQ => EmbeddingModel::SnowflakeArcticEmbedMLongQ,
            ModelChoice::SnowflakeArcticEmbedL => EmbeddingModel::SnowflakeArcticEmbedL,
            ModelChoice::SnowflakeArcticEmbedLQ => EmbeddingModel::SnowflakeArcticEmbedLQ,
        }
    }
}

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

    /// Embedding model to use for semantic similarity.
    #[arg(long, value_enum, default_value = "bge-base")]
    model: ModelChoice,

    /// Increase logging verbosity (-v for debug, -vv for trace).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// List all available embedding models and exit.
    #[arg(long)]
    list_models: bool,

    /// Disable the progress bar.
    #[arg(long)]
    no_progress: bool,

    /// One or more files to read input from. If provided, these take
    /// precedence over stdin and the clipboard; their contents are
    /// concatenated together.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
}

/// Maps a `-v` occurrence count to a tracing verbosity level.
fn verbosity_level(count: u8) -> Level {
    match count {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
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

/// Prints all available embedding models with their descriptions.
fn print_available_models() {
    for choice in ModelChoice::value_variants() {
        let possible_value = choice
            .to_possible_value()
            .expect("ModelChoice variants always have a possible value");
        let name = possible_value.get_name();
        let help = possible_value
            .get_help()
            .map(|help| help.to_string())
            .unwrap_or_default();
        println!("{name} - {help}");
    }
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

/// Reads and concatenates the contents of the given files, in order.
fn read_files(paths: &[PathBuf]) -> Result<String> {
    let mut contents = String::new();
    for path in paths {
        contents.push_str(&fs::read_to_string(path)?);
    }
    Ok(contents)
}

/// Reads input from the given files if any are provided, otherwise from
/// stdin if it is piped and non-empty, otherwise falls back to the system
/// clipboard.
fn read_input(files: &[PathBuf]) -> Result<String> {
    if !files.is_empty() {
        info!(count = files.len(), "reading input from files");
        return read_files(files);
    }

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

    if cli.list_models {
        print_available_models();
        return Ok(());
    }

    let raw_input = read_input(&cli.files)?;
    info!(bytes = raw_input.len(), "read input");

    let lines = input::read_non_blank_lines(&raw_input);

    let progress_enabled = !cli.no_progress && cli.verbose == 0 && io::stderr().is_terminal();
    let total_steps = lines.len() as u64;
    let progress = progress::Progress::new(total_steps, progress_enabled);

    let embeddings = embedding::embed_lines(&lines, cli.model.into(), &progress)?;
    let assignments = clustering::cluster(
        &embeddings,
        cli.min_cluster_size,
        cli.min_samples,
        &progress,
    )?;
    progress.finish_and_clear();

    let grouped = grouping::group(lines, assignments);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    output::print(&grouped, &mut handle)?;

    Ok(())
}
