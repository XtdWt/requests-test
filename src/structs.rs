use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct RepoData {
    pub name: String,
    pub url: String,
    pub description: String,
    pub language: String,
    pub created_date: String,
    pub last_updated: String,
    pub commits: i32,
}
