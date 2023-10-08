use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct SorterCmds {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new sorter rule
    New,

    /// View all or a specific sorter rule
    View,

    /// View a specific sorter rule
    Delete,

    /// Activate all sorters in background
    Up,

    /// Start sorters in foreground
    Serve,

    /// Deactivate all sorters running in background
    Down,
}
