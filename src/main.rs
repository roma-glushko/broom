use clap::Parser;

pub mod cmd;

fn main() {
    let cli = cmd::Cli::parse();

    match &cli.command {
        Some(cmd::Commands::Clean(name)) => {
            match name.plan_name {
                Some(ref _name) => {
                    println!("reverse")
                }
                None => {
                    println!("please provide a string to reverse");
                }
            }
        }
        None => {}
        &Some(cmd::Commands::PlanNew(_)) | &Some(cmd::Commands::PlanView(_)) | &Some(cmd::Commands::PlanDelete(_)) => todo!()
    }
}
