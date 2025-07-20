use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use tokio_util::io::StreamReader;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures_util::TryStreamExt;

#[derive(Debug, Deserialize)]
struct PromptRequest {
    url: String,
    model: String,
    prompt: String,
}

#[derive(Debug, Serialize)]
struct PromptResponse {
    response: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_index))
        .nest_service("/public", ServeDir::new("public"))
        .route("/api/prompt", post(send_prompt))
        .route("/api/models", get(get_models))
        .route("/health", get(health_check))
        .route("/status", get(status_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🌐 Servidor escuchando en http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn serve_index() -> impl IntoResponse {
    match tokio::fs::read_to_string("public/index.html").await {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "❌ Error al cargar index.html",
        )
            .into_response(),
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(Serialize)]
struct StatusInfo {
    status: &'static str,
    message: &'static str,
}

async fn status_check() -> impl IntoResponse {
    let info = StatusInfo {
        status: "OK",
        message: "Servidor funcionando correctamente",
    };
    (StatusCode::OK, Json(info))
}

async fn send_prompt(Json(payload): Json<PromptRequest>) -> impl IntoResponse {
    let url = format!("{}/api/generate", payload.url.trim_end_matches('/'));
    let client = Client::new();

    let body = serde_json::json!({
        "model": payload.model,
        "prompt": payload.prompt,
    });

    match client.post(&url).json(&body).send().await {
        Ok(resp) => {
            let stream = resp.bytes_stream();
            let stream_reader = StreamReader::new(
                stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
            );
            let mut lines = BufReader::new(stream_reader).lines();

            let mut answer = String::new();

            while let Some(result) = lines.next_line().await.transpose() {
                match result {
                    Ok(line) => {
                        if let Ok(json_line) = serde_json::from_str::<serde_json::Value>(&line) {
                            if let Some(fragment) = json_line.get("response").and_then(|v| v.as_str()) {
                                answer.push_str(fragment);
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Error leyendo línea del stream: {:?}", e);
                        break;
                    }
                }
            }

            (StatusCode::OK, Json(PromptResponse { response: answer }))
        }
        Err(e) => {
            println!("❌ Error enviando al modelo: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PromptResponse {
                    response: "❌ Error enviando el prompt al modelo.".to_string(),
                }),
            )
        }
    }
}

#[derive(Deserialize)]
struct ModelQuery {
    url: String,
}

async fn get_models(Query(params): Query<ModelQuery>) -> impl IntoResponse {
    let client = Client::new();
    let url = format!("{}/api/tags", params.url.trim_end_matches('/'));

    match client.get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(json) => {
                let models: Vec<String> = json["models"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|m| m["name"].as_str().map(String::from))
                    .collect();
                (StatusCode::OK, Json(models))
            }
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<String>::new()),
            ),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Vec::<String>::new()),
        ),
    }
}

