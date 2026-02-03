mod structs;
mod response_types;

use tokio;
use reqwest;
use std::error::Error;
use structs::RepoData;
use response_types::{GithubReposResponse, GithubCommitResponse};


static USERNAME: &str = "XtdWt";


async fn make_github_get_request(client: &reqwest::Client, url: &str) -> reqwest::Response {
    let resp = client.get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", USERNAME)
        .send()
        .await
        .unwrap_or_else(|e| panic!("error sending request to {}: {}", url, e));
    resp
}


async fn create_repo_data(user: &str) -> Result<Vec<RepoData>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = String::from("https://api.github.com");
    let users_url = base_url + "/users/";
    let request_url = users_url + user + "/repos";

    let mut resp = make_github_get_request(&client, &request_url).await;
    let resp_data: Vec<GithubReposResponse> = resp.json().await?;
    println!("Found repositories data for {:#?}", resp_data.len());
    let mut repo_data_list = Vec::new();
    for repo_data in resp_data.iter() {
        let commits_url = String::from("https://api.github.com/repos/XtdWt/") + &repo_data.name + "/commits";
        resp = make_github_get_request(&client, commits_url.as_str()).await;
        let commit_data: Vec<GithubCommitResponse> = resp.json().await?;
        let created_at_str = repo_data.created_at
            .clone()
            .unwrap_or_else(|| "YYYY-MM-DD".to_string());
        let created_at_str = &created_at_str[0..10];
        let updated_at_str = repo_data.updated_at
            .clone()
            .unwrap_or_else(|| "YYYY-MM-DD".to_string());
        let updated_at_str = &updated_at_str[0..10];
        let repo_data = RepoData {
            name: repo_data.name.clone(),
            url: repo_data.url.clone(),
            description: repo_data.description.clone().unwrap_or_else(|| "".to_string()),
            language: repo_data.language.clone().unwrap_or_else(|| "".to_string()),
            created_date: created_at_str.parse()?,
            last_updated: updated_at_str.parse()?,
            commits: commit_data.len() as i32,
        };
        repo_data_list.push(repo_data);
    };
    Ok(repo_data_list)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repo_data_list = create_repo_data(USERNAME).await?;
    println!("{:#?}", repo_data_list);
    Ok(())
}
