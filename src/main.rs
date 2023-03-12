use clap::{Parser, Subcommand};
use std::{env, io, str};
use url::Url;

const BITBUCKET_HOST: &str = "bitbucket.org";

mod git_config {
    use std::{path::PathBuf, io, fs};

    use url::Url;

    const BITBUCKET_HOST: &str = "bitbucket.org";

    #[derive(Debug)]
    pub struct GitConfig {
        remotes: Vec<GitRemote>,
    }


    #[derive(Debug)]
    pub struct GitRemote {
        pub url: String,
    }

    impl GitConfig {
        pub fn find_bitbucket_remote(&self) -> Option<&GitRemote> {
            let mut found_remote: Option<&GitRemote> = None;
            for remote in self.remotes.iter() {
                let remote_url = Url::parse(&remote.url).expect("Invalid URL");
                if remote_url.host_str() == Some(BITBUCKET_HOST) {
                    found_remote = Some(remote);
                }
            }
            found_remote
        }
    }

    pub fn parse_local(dir: PathBuf) -> Result<GitConfig, io::Error> {
        let git_config_path = dir.join(".git").join("config");
        let git_config = fs::read_to_string(git_config_path);

        let config_data = match git_config {
            Ok(config) => config,
            Err(e) => return Err(e)
        };
        
        let mut remotes: Vec<GitRemote> = Vec::new();
        for attr in config_data.split("[") {
            let config_set = attr.replace("\t", "").replace("]", "");
            let config_set: Vec<&str> = config_set.split("\n").collect();

            let header = config_set[0];
            if header.starts_with("remote") {
                let remote_options: Vec<&str> = header.split(" ").collect();
                let url_options: Vec<&str> = config_set[1].split(" = ").collect();
                let fetch_options: Vec<&str> = config_set[2].split(" = ").collect();
                let name = remote_options[1].replace("\"", "");
                let url = url_options[1].to_string();
                let fetch = fetch_options[1].to_string();
                remotes.push(GitRemote {
                    name, 
                    url, 
                    fetch,
                })
            }
        };

        Ok(GitConfig {
            remotes
        })
    }
}

mod BitBucketRepo {
    use url::Url;

    pub struct BitBucketRepo {
        pub name: String,
        pub workspace: String,
    }

    pub fn new(name: String, workspace: String) -> BitBucketRepo {
        BitBucketRepo { name, workspace }
    }

    pub fn from_remote_url(url: &str) -> BitBucketRepo {
        let url = Url::parse(&url).expect("Invalid URL");
        let path: Vec<&str> = url.path().split("/").collect();
        let workspace = path[1].to_string();
        let name = path[2].replace(".git", "").to_string();
    
        BitBucketRepo { name, workspace }
    }
}

#[derive(Parser)]
#[command(
    author,
    version,
    about = "An unofficial command-line tool for working with BitBucket."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Browse {
        #[arg(value_name = "branch", short, long)]
        branch: Option<String>,

        #[arg(value_name = "commit", short, long)]
        commit: Option<String>,

        #[arg(value_name = "no_browser", short, long)]
        no_browser: bool,

        #[arg(value_name = "repo", short, long)]
        repo: Option<String>,

        #[arg(value_name = "settings", short, long)]
        settings: Option<String>
    },
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
                    BitBucketRepo::new(repo_option[1].to_string(), repo_option[0].to_string())
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
                    BitBucketRepo::from_remote_url(bitbucket_remote.url.as_str())
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
