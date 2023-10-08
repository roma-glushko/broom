use clap::{Parser, Subcommand};

use crate::lifecycle::cmds::LifecycleCmds;
use crate::sorters::cmds::SorterCmds;

#[derive(Parser)]
#[command(author, author, version)]
#[command(about = "🧹 broom - clean your temporary files based on your cleaning plans with ease")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Config,
    /// Manage lifecycle rules
    Lifecycle(LifecycleCmds),

    /// Sort files you create in real time
    Sorter(SorterCmds),
}
