use std::env;
use std::io::{self};
use std::process::Command;

use axum::extract::Query;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

fn download_audio_from_youtube(url: String, output: &str) -> io::Result<()> {
    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("m4a")
        .arg("-o")
        .arg(output)
        .arg(url)
        .status()?;

    if !status.success() {
        eprintln!("Error downloading audio.");
        std::process::exit(1);
    }
    Ok(())
}

async fn download_end_point(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response<axum::body::Body>, (axum::http::StatusCode, String)> {
    println!("video id is {:?}", params);
    let video_id: String = match params.get("video_id") {
        Some(video_id) => video_id.clone(),
        None => "".to_string(),
    };
    let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);
    let download_path_string = format!("./{}.%(ext)s", &video_id);
    let download_path = download_path_string.as_str();

    // Download audio
    if let Err(e) = download_audio_from_youtube(video_url, download_path) {
        eprintln!("Error downloading audio: {}", e);
    }

    let mut file = tokio::fs::File::open(format!("./{}.m4a", &video_id))
        .await
        .expect("Failed to open video file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .expect("Failed to read video file");

    let response = Response::builder()
        .header("Content-Type", "audio/mp4") // Correct MIME type for .m4a (audio/mp4)
        .header(
            "Content-Disposition",
            format!("inline; filename=\"./{}.m4a\"", &video_id),
        ) // Optional: suggest file download or inline playback
        .body(axum::body::Body::from(buffer))
        .unwrap();

    tokio::spawn(async move { std::fs::remove_file(format!("{}.m4a", &video_id)) });

    Ok(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/download", get(download_end_point))
        .route("/", get("ok"));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
