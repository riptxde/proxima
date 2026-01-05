use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PastVersionsResponse {
    #[serde(rename = "Windows")]
    pub windows: String,
    #[serde(rename = "WindowsDate")]
    pub windows_date: String,
    #[serde(rename = "Mac")]
    pub mac: String,
    #[serde(rename = "MacDate")]
    pub mac_date: String,
}

#[tauri::command]
pub async fn launcher_fetch_past_versions() -> Result<PastVersionsResponse, String> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://weao.xyz/api/versions/past")
        .header("User-Agent", "WEAO-3PService")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch past versions: {}", e))?;

    let data = response
        .json::<PastVersionsResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}
