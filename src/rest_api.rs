use anyhow::bail;
use serde::Deserialize;
use thiserror::Error;

use crate::shared::Repository;

const BODY: &str = r#"
{
    "name": "web",
    "active": true,
    "events": [
        "issues",
        "pull_request"
    ],
    "config": {
        "url": "http://188.68.57.24:8080/receive",
        "content_type": "json",
        "insecure_ssl": "0"
    }
}
"#;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("Unknown error")]
    Unknown,

    #[error("The hook already exists")]
    AlreadyExists,
}

#[derive(Deserialize)]
struct CreatedResponse {
    pub id: usize,
}

pub async fn create_hook(owner: &str, repo: &str, key: &str) -> anyhow::Result<usize> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{owner}/{repo}/hooks");
    let res = client
        .post(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {key}"))
        .header("User-Agent", "deltachat-github-bot")
        .body(BODY)
        .send()
        .await?;

    let status = res.status();
    if status == 201 {
        let resp = serde_json::from_str::<CreatedResponse>(&res.text().await?)?;
        Ok(resp.id)
    } else if status == 422 {
        Err(HookError::AlreadyExists)?
    } else {
        Err(HookError::Unknown)?
    }
}

pub async fn remove_hook(owner: &str, repo: &str, hook: usize, key: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{owner}/{repo}/hooks/{hook}");
    let res = client
        .delete(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {key}"))
        .header("User-Agent", "deltachat-github-bot")
        .send()
        .await?;
    if res.status() == 204 {
        Ok(())
    } else {
        bail!("something went wrong: {}", res.status())
    }
}

pub async fn get_repository(owner: &str, repo: &str, key: &str) -> anyhow::Result<Repository> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{owner}/{repo}");
    let res = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {key}"))
        .header("User-Agent", "deltachat-github-bot")
        .send()
        .await?;
    Ok(serde_json::from_str::<Repository>(&res.text().await?)?)
}

/*
// at some point it would be nice to have tests here

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_repository() {
        let repo = get_repository(
            "septias",
            "github-bot",
            "<secret>",
        )
        .await
        .unwrap();
        println!("{repo:?}")
        assert_eq!(repo, Repository { id: 558781383, name: "github-bot", url: "https://api.github.com/repos/Septias/github-bot" })
    }
}
 */
