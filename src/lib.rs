pub mod log;
pub mod models;
pub mod package_manager;
pub mod utils;

use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Subcommand, Clone, Debug)]
pub enum Action {
    Search { value: Regex },
}

#[derive(Parser, Clone, Debug)]
#[clap(author, name = "Mono Check")]
pub struct Args {
    /// Ignore matching package names
    #[clap(short, value_parser, long)]
    pub ignore: Option<Vec<String>>,

    /// Filter by matching package name
    #[clap(short, value_parser, long)]
    pub matches: Option<Regex>,

    /// Minimum number of workspaces
    /// to include
    #[clap(value_parser, long, default_value_t = 2)]
    pub min: usize,

    /// Ignore matching workspaces names
    #[clap(value_parser, long, short = 'I')]
    pub ignore_workspace: Vec<String>,

    /// Ignore matching workspaces names
    #[clap(value_parser, global = true, long, short = 'R')]
    pub include_root: bool,

    /// Filter by matching workspace name
    #[clap(value_parser, long, short = 'M')]
    pub match_workspace: Option<Regex>,

    /// Check for version differences in dependencies
    #[clap(long, global = true)]
    pub deep: bool,

    /// Output as JSON (deep by default)
    #[clap(global = true, long, value_parser)]
    pub json: bool,

    /// Output as YAML (deep by default)
    #[clap(global = true, long, value_parser)]
    pub yaml: bool,

    #[clap(long, value_parser)]
    pub no_color: bool,

    #[clap(global = true, long, short = 'D', value_parser)]
    pub dev: bool,

    #[clap(global = true, long, value_parser)]
    pub peer: bool,

    #[clap(global = true, long, short = 'P', value_parser)]
    pub prod: bool,

    #[clap(long, short = 'W', value_parser)]
    pub check_workspace: bool,

    #[clap(subcommand)]
    pub action: Option<Action>,
}
