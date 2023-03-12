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