use axum::{
    routing::get,
    extract::{Query, Request},
    response::IntoResponse,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeFile;
use tower_http::cors::CorsLayer;
use std::path::PathBuf;
use tower::util::ServiceExt; // Required for oneshot

#[derive(Deserialize)]
struct VideoParams {
    file: String,
}

pub async fn start_server() -> u16 {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    
    let app = Router::new()
        .route("/stream", get(stream_handler))
        .route("/subtitle", get(subtitle_handler))
        .layer(CorsLayer::permissive());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    println!("🚀 Local Video Server running on port: {}", port);
    port
}

async fn stream_handler(Query(params): Query<VideoParams>, req: Request) -> impl IntoResponse {
    let path = PathBuf::from(params.file);
    if !path.exists() {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    }

    // ServeFile comes with built-in Range support (206 Partial Content)
    // We forward the request headers (Range) to ServeFile
    let service = ServeFile::new(path);
    
    match service.oneshot(req).await {
        Ok(res) => res.into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("File streaming error: {}", err),
        )
            .into_response(),
    }
}

async fn subtitle_handler(Query(params): Query<VideoParams>) -> impl IntoResponse {
    let video_path = PathBuf::from(&params.file);

    // Try .srt first, then .vtt
    let srt_path = video_path.with_extension("srt");
    let vtt_path = video_path.with_extension("vtt");

    if vtt_path.exists() {
        // Serve .vtt directly
        let content = match std::fs::read_to_string(&vtt_path) {
            Ok(c) => c,
            Err(_) => return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        return (
            [(axum::http::header::CONTENT_TYPE, "text/vtt; charset=utf-8")],
            content,
        ).into_response();
    }

    if srt_path.exists() {
        // Convert .srt → .vtt on the fly
        let srt = match std::fs::read_to_string(&srt_path) {
            Ok(c) => c,
            Err(_) => return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let mut vtt = String::from("WEBVTT\n\n");
        for line in srt.lines() {
            if line.contains(" --> ") {
                vtt.push_str(&line.replace(',', "."));
            } else {
                vtt.push_str(line);
            }
            vtt.push('\n');
        }

        return (
            [(axum::http::header::CONTENT_TYPE, "text/vtt; charset=utf-8")],
            vtt,
        ).into_response();
    }

    axum::http::StatusCode::NOT_FOUND.into_response()
}
