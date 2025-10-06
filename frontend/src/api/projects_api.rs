use shared::{Project, ProjectsResponse, ProjectSearchRequest};
use super::client;

pub async fn list_projects(
    page: Option<usize>,
    per_page: Option<usize>,
    category: Option<String>,
) -> Result<ProjectsResponse, String> {
    let mut url = "/api/projects?".to_string();

    if let Some(p) = page {
        url.push_str(&format!("page={}&", p));
    }
    if let Some(pp) = per_page {
        url.push_str(&format!("per_page={}&", pp));
    }
    if let Some(cat) = category {
        url.push_str(&format!("category={}&", cat));
    }

    client::get(&url).await
}

pub async fn get_project(id: &str) -> Result<Project, String> {
    client::get(&format!("/api/projects/{}", id)).await
}

pub async fn search_projects(request: ProjectSearchRequest) -> Result<Vec<Project>, String> {
    client::post("/api/projects/search", &request).await
}
