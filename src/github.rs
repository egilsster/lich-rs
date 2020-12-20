use std::path::Path;

use reqwest::header::HeaderMap;
use serde::Deserialize;

use crate::types::DependencyList;

use eyre::{Result, WrapErr};

#[derive(Deserialize, Debug)]
struct TreeItem {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    _type: String,
    sha: Option<String>,
    size: Option<i64>,
    url: String,
}

#[derive(Deserialize, Debug)]
struct GithubRepoResponse {
    sha: String,
    url: String,
    tree: Vec<TreeItem>,
}

const FILE_EXT: &'static str = ".toml";

pub async fn get_approved_dependencies(
    github_url: String,
    github_token: Option<String>,
) -> Result<DependencyList> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "github/egilsster/lich".parse()?);
    if let Some(token) = github_token {
        headers.insert("Authorization", format!("token {}", token).parse()?);
    }

    let req = reqwest::Client::new();
    let resp = req
        .get(&github_url)
        .headers(headers)
        .send()
        .await
        .wrap_err("Github request did not complete successfully")?
        .text()
        .await?;

    let data: GithubRepoResponse = serde_json::from_str(&resp)
        .wrap_err("Github response was not what was expected. Check your token and try again.")?;

    let mut approved_dependencies = DependencyList::new();
    for item in data.tree {
        if item._type.eq("blob") && item.path.ends_with(FILE_EXT) {
            let filename = Path::new(&item.path).file_stem().unwrap();
            approved_dependencies.push(filename.to_str().unwrap().to_string());
        }
    }

    Ok(approved_dependencies)
}

/// Get parameters from parameters to build the url
pub fn get_project_api_url(owner: &str, repo: &str, branch: &str) -> String {
    return format!(
        "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
        owner, repo, branch
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_api_url() {
        assert_eq!(
            "https://api.github.com/repos/egilsster/approved-licenses/git/trees/main?recursive=1",
            get_project_api_url("egilsster", "approved-licenses", "main")
        );
    }
}
