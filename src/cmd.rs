use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, author, version)]
#[command(about = "🧹 broom - clean your temporary files based on your cleaning plans with ease")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform cleaning according to the plan
    Clean(Clean),
    /// Create a new cleaning plan
    PlanNew(PlanNew),
    /// View all or a specific cleaning plan
    PlanView(PlanView),
    /// View a specific cleaning plan
    PlanDelete(PlanDelete),
}

#[derive(Args)]
pub struct Clean {
    /// A plan name to execute
    pub plan_name: Option<String>,
}

#[derive(Args)]
pub struct PlanNew {
    /// Plan name
    pub name: Option<String>,
}

#[derive(Args)]
pub struct PlanView {
    /// Plan name
    pub name: Option<String>,
}

#[derive(Args)]
pub struct PlanDelete {
    /// Plan name
    pub name: Option<String>,
}

// #[derive(Args)]
// pub struct Inspect {
//     /// The string to inspect
//     pub string: Option<String>,
//     #[arg(short = 'd', long = "digits")]
//     only_digits: bool,
// }
