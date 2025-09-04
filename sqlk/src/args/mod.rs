use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the SQL file to open
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Path to the .env file (default: .env)
    #[arg(short, long, default_value = ".env")]
    pub env: PathBuf,

    /// Disable matrix loading animation
    #[arg(long)]
    pub no_matrix: bool,

    /// Change toast level
    #[arg(short, long, default_value = "INFO")]
    pub toast_level: String,

    /// Direct SQL query to execute
    #[arg(short, long)]
    pub query: Option<String>,
}
