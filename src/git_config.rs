use std::{path::PathBuf, io, fs};

use url::Url;

const BITBUCKET_HOST: &str = "bitbucket.org";

#[derive(Debug)]
pub struct GitConfig {
    remotes: Vec<GitRemote>,
}


#[derive(Debug)]
pub struct GitRemote {
    name: String,
    pub url: String,
    fetch: String,
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