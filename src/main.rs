use clap::{Parser, Subcommand};
use std::{env, io, str};
use url::Url;

const BITBUCKET_HOST: &str = "bitbucket.org";

mod git_config;
mod bitbucket_repo;

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
            let cwd = env::current_dir().expect("Can't read current dir");

            let bb_repo = match repo {
                Some(r) => {
                    // TODO: Validate repo flag
                    let repo_option: Vec<&str> = r.split("/").collect();
                    bitbucket_repo::new(repo_option[1].to_string(), repo_option[0].to_string())
                },
                None => {
                    // Parse local git config for remotes
                    let git_config = match git_config::parse_local(cwd) {
                        Ok(config) => config,
                        Err(e) => return match e.kind() {
                            io::ErrorKind::NotFound => {
                                println!("Error while parsing Git config: ./git/config not found")
                            },
                            _ => println!("Unexpected error while parsing Git config: {}", e),
                        }
                    };

                    // Find BitBucket remote repository in config
                    let bitbucket_remote = match git_config.find_bitbucket_remote() {
                        Some(remote) => remote,
                        None => return println!("Project does not appear to be a repository on BitBucket.")
                    };
                    bitbucket_repo::from_remote_url(bitbucket_remote.url.as_str())
                }
            };

            // Set default browse URL (https://bitbucket.org/<workspace>/<repo_name>)
            let mut browse_url = Url::parse(format!("https://{}", BITBUCKET_HOST).as_str()).expect("Unable to parse Bitbucket host");
            browse_url.path_segments_mut().expect("")
                .push(&bb_repo.workspace)
                .push(&bb_repo.name);

            // Handle browse by branch
            if branch.is_some() {
                let branch = branch.unwrap();
                browse_url.path_segments_mut().unwrap().push("branch").push(&branch.as_str());
            }

            // Handle browse by commit
            if commit.is_some() {
                let commit = commit.unwrap();
                browse_url.path_segments_mut().unwrap().push("src").push(&commit.as_str());
            }

            // Handle --no-browser flag
            match no_browser {
                true => println!("{}", browse_url.as_str()),
                false => webbrowser::open(browse_url.as_str()).expect("Could not open remote URL")
            }
            
        }
        Commands::Clone { remote } => {
            println!("Not implemented");
        }
    }
}
