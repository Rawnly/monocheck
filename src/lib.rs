pub mod log;
pub mod models;
pub mod utils;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Clone, Debug)]
#[clap(version = "0.1.0", author, name = "Mono Check")]
pub struct Args {
    /// Ignore matching package names
    #[clap(short, value_parser, long, global = true)]
    pub ignore: Option<Vec<String>>,

    /// Filter by matching package name
    #[clap(short, value_parser, long, global = true)]
    pub matches: Option<Regex>,

    /// Minimum number of workspaces
    /// to include
    #[clap(value_parser, long, default_value_t = 2)]
    pub min: usize,

    /// Ignore matching workspaces names
    #[clap(value_parser, long, short = 'I')]
    pub ignore_workspace: Vec<String>,

    /// Filter by matching workspace name
    #[clap(value_parser, long, short = 'M')]
    pub match_workspace: Option<Regex>,

    /// Check for version differences in dependencies
    #[clap(long, global = true)]
    pub deep: bool,

    /// Output as JSON
    #[clap(long, value_parser)]
    pub json: bool,
}
