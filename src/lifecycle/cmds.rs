use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct LifecycleCmds {
    #[clap(subcommand)]
    command: LifecycleCommands,
}

#[derive(Subcommand, Debug)]
enum LifecycleCommands {
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
    /// Lifecycle Policy Name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleView {
    /// Lifecycle Policy Name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleDelete {
    /// Lifecycle Policy Name
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct LifecycleApply {
    /// Lifecycle Policy Name to Execute
    pub name: Option<String>,
}
