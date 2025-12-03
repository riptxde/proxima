use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::{Filter, Reply};

#[derive(Serialize, Deserialize, Clone)]
struct HttpExecutePayload {
    script: String,
    source: String, // "http_file" or "http_direct"
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

/// Start the HTTP server on port 13377
pub async fn start_http_server(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:13377".parse()?;

    log_ui!(&app_handle, Success, "HTTP server started on port 13377");

    // /execute_file endpoint
    let app_execute_file = app_handle.clone();
    let execute_file_route = warp::path("execute_file")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(move |body: Bytes| {
            let app = app_execute_file.clone();
            async move { handle_execute_file(body, app).await }
        });

    // /execute endpoint
    let app_execute = app_handle.clone();
    let execute_route = warp::path("execute")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(move |body: Bytes| {
            let app = app_execute.clone();
            async move { handle_execute(body, app).await }
        });

    let routes = execute_file_route.or(execute_route);

    warp::serve(routes).run(addr).await;

    Ok(())
}

/// Handle /execute_file endpoint
async fn handle_execute_file(body: Bytes, app: AppHandle) -> Result<impl Reply, warp::Rejection> {
    let path_str = String::from_utf8_lossy(&body).trim().to_string();

    if path_str.is_empty() {
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: "File path cannot be empty".to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Parse path (no security restrictions - allow any file path)
    let file_path = Path::new(&path_str);

    // Check if file exists
    if !file_path.exists() {
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: format!("File not found: {}", path_str),
            }),
            StatusCode::NOT_FOUND,
        ));
    }

    // Validate extension
    if let Err(e) = validate_extension(file_path) {
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: e,
            }),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Read file content
    let script = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    success: false,
                    error: format!("Failed to read file: {}", e),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    // Emit event to frontend
    let payload = HttpExecutePayload {
        script,
        source: "http_file".to_string(),
    };

    if let Err(e) = app.emit("http-execute-script", payload) {
        log::error!("Failed to emit http-execute-script event: {}", e);
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: "Internal server error".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    log::info!("HTTP request received: execute_file ({})", path_str);

    Ok(with_status(
        json(&SuccessResponse {
            success: true,
            message: "Script sent for execution".to_string(),
        }),
        StatusCode::OK,
    ))
}

/// Handle /execute endpoint
async fn handle_execute(body: Bytes, app: AppHandle) -> Result<impl Reply, warp::Rejection> {
    let script = String::from_utf8_lossy(&body).trim().to_string();

    if script.is_empty() {
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: "Script cannot be empty".to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Emit event to frontend
    let payload = HttpExecutePayload {
        script,
        source: "http_direct".to_string(),
    };

    if let Err(e) = app.emit("http-execute-script", payload) {
        log::error!("Failed to emit http-execute-script event: {}", e);
        return Ok(with_status(
            json(&ErrorResponse {
                success: false,
                error: "Internal server error".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    log::info!("HTTP request received: execute");

    Ok(with_status(
        json(&SuccessResponse {
            success: true,
            message: "Script sent for execution".to_string(),
        }),
        StatusCode::OK,
    ))
}

/// Validate file extension
fn validate_extension(path: &Path) -> Result<(), String> {
    let valid_extensions = ["lua", "luau", "txt"];

    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            if valid_extensions.contains(&ext) {
                Ok(())
            } else {
                Err(format!("Invalid file extension: .{}", ext))
            }
        }
        None => Err("File has no extension".to_string()),
    }
}
