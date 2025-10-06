use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "http://localhost:3000";

pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
    endpoint: &str,
    body: &T,
) -> Result<R, String> {
    let url = format!("{}{}", API_URL, endpoint);

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}

pub async fn get<R: for<'de> Deserialize<'de>>(endpoint: &str) -> Result<R, String> {
    let url = format!("{}{}", API_URL, endpoint);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}

#[derive(Deserialize)]
struct ApiResponse<T> {
    data: T,
    #[allow(dead_code)]
    message: String,
}

async fn handle_response<R: for<'de> Deserialize<'de>>(
    response: Response,
) -> Result<R, String> {
    if response.ok() {
        let wrapper = response
            .json::<ApiResponse<R>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(wrapper.data)
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        Err(format!("HTTP {}: {}", status, error_text))
    }
}
