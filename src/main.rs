use std::path::PathBuf;
use clap::{Parser, Subcommand, command, arg};

mod models;
mod project;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "prismarine")]
struct CliArgs {
    #[command(subcommand)]
    commands: Commands
}


#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Creates a new project in either a new folder or a specified directory")]
    New {
        #[arg(value_name = "PROJECT_NAME", help = "Project's name")]
        name: String,
        #[arg(value_name = "PATH", help = "Project path (defaults to new directory of project name)")]
        path: Option<PathBuf>
    },
    #[command(about = "Exports the pack as a zip file")]
    Export,
    #[command(about = "Returns info about a resource pack.")]
    Info {
        #[arg(long, short)]
        name: Option<bool>
    }
}


fn main() {
    dotenvy::dotenv().ok();

    let args = CliArgs::parse();

    match args.commands {
        Commands::New { name, path } => {
            project::create_project(name.clone(), path.clone());
        },
        Commands::Export => {
            project::export_project();
        },
        Commands::Info { name } => {
            let _ = name;
            todo!("If prismarine.toml is present in current directory or supplied directory, print info from it")
        }
    }
}
