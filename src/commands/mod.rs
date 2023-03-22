use std::{env, io};

use url::Url;

use crate::{bitbucket_repo, git_config, BITBUCKET_HOST};

pub fn browse(branch: Option<String>, commit: Option<String>, no_browser: bool, repo: Option<String>, settings: Option<String>) {
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