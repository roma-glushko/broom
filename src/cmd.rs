use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, author, version)]
#[command(about = "🧹 broom - clean your temporary files based on your cleaning plans with ease")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage lifecycle rules
    Lifecycle(Lifecycle),
    /// Sort files you create in real time
    Sorter,
}

#[derive(Args, Debug)]
pub struct Lifecycle {
    #[clap(subcommand)]
    command: LifecycleCommands,
}

#[derive(Subcommand, Debug)]
pub enum LifecycleCommands {
    /// Create a new lifecycle plan
    New(LifecycleNew),

    /// View all or a specific lifecycle plan
    View(LifecycleView),

    /// View a specific lifecycle plan
    Delete(LifecycleDelete),

    /// Perform cleaning according to lifecycle rules
    Apply(LifecycleApply),
}

#[derive(Args, Debug)]
pub struct LifecycleNew {
    /// Plan name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleView {
    /// Plan name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleDelete {
    /// Plan name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleApply {
    /// A plan name to execute
    pub plan_name: Option<String>,
}

#[derive(Args, Debug)]
pub struct Sorter {
    #[clap(subcommand)]
    command: SorterCommands,
}

#[derive(Subcommand, Debug)]
pub enum SorterCommands {
    /// Create a new sorter rule
    New,

    /// View all or a specific sorter rule
    View,

    /// View a specific sorter rule
    Delete,

    /// Activate all sorters
    Up,

    /// Deactivate all sorters
    Down,
}
