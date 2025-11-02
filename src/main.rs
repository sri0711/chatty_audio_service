use axum::extract::Query;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::env;
use std::io::{self};
use std::path::Path;
use std::process::Command;
use tokio::fs;
use tokio::io::AsyncReadExt;

fn download_audio_from_youtube(url: String, output: &str) -> io::Result<()> {
    Command::new("yt-dlp").arg("-U");
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
        return Err(io::Error::new(io::ErrorKind::Other, "Download failed"));
    }
    Ok(())
}

async fn download_end_point(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response<axum::body::Body>, (axum::http::StatusCode, String)> {
    println!("video id is {:?}", params);
    let video_id: String = match params.get("video_id") {
        Some(video_id) => video_id.clone(),
        None => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Video ID is required".to_string(),
            ))
        }
    };

    let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);
    let download_path_string = format!("./{}.%(ext)s", &video_id);
    let download_path = download_path_string.as_str();

    // Ensure the directory exists before downloading
    let output_dir = Path::new(&download_path_string).parent().unwrap();
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .await
            .expect("Failed to create output directory");
    }

    // Download audio
    if let Err(e) = download_audio_from_youtube(video_url, download_path) {
        eprintln!("Error downloading audio: {}", e);
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Download failed".to_string(),
        ));
    }

    let file_path = format!("{}.m4a", &video_id);
    if !Path::new(&file_path).exists() {
        return Err((
            axum::http::StatusCode::NOT_FOUND,
            "Audio file not found".to_string(),
        ));
    }

    let mut file = fs::File::open(&file_path).await.map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to open audio file".to_string(),
        )
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read audio file".to_string(),
        )
    })?;

    let response = Response::builder()
        .header("Content-Type", "audio/mp4") // Correct MIME type for .m4a (audio/mp4)
        .header(
            "Content-Disposition",
            format!("inline; filename=\"{}.m4a\"", &video_id),
        ) // Optional: suggest file download or inline playback
        .body(axum::body::Body::from(buffer))
        .unwrap();

    // Remove the file after the response is sent
    tokio::spawn(async move {
        if let Err(e) = fs::remove_file(file_path).await {
            eprintln!("Failed to remove temporary file: {}", e);
        }
    });

    Ok(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/download", get(download_end_point))
        .route("/", get(|| async { "ok" }));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("server is started.");
    axum::serve(listener, app).await.unwrap();
}
