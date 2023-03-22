use clap::{Parser, Subcommand};
use std::{str};

const BITBUCKET_HOST: &str = "bitbucket.org";

mod git_config;
mod bitbucket_repo;
mod commands;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "An unofficial command-line tool for interacting with BitBucket repositories."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open the repository in the browser
    Browse {
        /// Select another branch by passing in the branch name
        #[arg(value_name = "branch", short, long)]
        branch: Option<String>,

        /// Open source code at a specific commit
        #[arg(value_name = "commit", short, long)]
        commit: Option<String>,

        /// Print destination URL instead of opening the browser
        #[arg(value_name = "no_browser", short, long)]
        no_browser: bool,

        /// Select another repository using the WORKSPACE/REPO format
        #[arg(value_name = "repo", short, long)]
        repo: Option<String>,

        /// Not implemented, will be ignored
        #[arg(value_name = "settings", short, long)]
        settings: Option<String>
    },
    /// Clone a repository locally
    Clone {
        remote: String
    }
}

#[allow(unused_variables)]
fn main() {
    let args = Cli::parse();
    
    match args.command { 
        Commands::Browse { 
            branch, 
            commit, 
            no_browser, 
            repo,
            settings 
        } => {
            commands::browse(branch, commit, no_browser, repo, settings)
        }
        Commands::Clone { remote } => {
            println!("Not implemented");
        }
    }
}
