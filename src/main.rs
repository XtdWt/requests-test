mod response_types;
mod structs;

use std::error::Error;
use tokio;
use reqwest;
use reqwest::Response;
use response_types::{GithubUserResponse, GithubReposResponse};

static USERNAME: &str = "XtdWt";


async fn make_github_get_request(client: &reqwest::Client, url: &str) -> Response {
    let resp = client.get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", USERNAME)
        .send()
        .await
        .unwrap_or_else(|e| panic!("error sending request to {}: {}", url, e));
    resp
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let base_url = String::from("https://api.github.com");
    let users_url = base_url + "/users/";
    let request_url = users_url + USERNAME;
    let resp = make_github_get_request(&client, request_url.as_str()).await;
    let resp_data: GithubUserResponse = resp.json().await?;
    println!("Found user data for {:#?}", resp_data.get_username());

    let repos_url = resp_data.get_repositories_url();
    let resp = make_github_get_request(&client, repos_url.as_str()).await;
    let resp_data: Vec<GithubReposResponse> = resp.json().await?;
    println!("{:#?}", resp_data);
    Ok(())
}
