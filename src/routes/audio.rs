pub mod audio_routes {

    // imports
    use axum::{extract::Query, routing::get, Json, Router};
    use reqwest;
    use serde::Deserialize;
    use serde_json::{json, Value};

    use crate::helper::config::configurations::get_configs;

    // common structs
    #[derive(Deserialize, Debug)]
    struct SearchSongQuery {
        // make optional so missing param doesn't crash extractor
        search_input: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct SongDetailsQuery {
        id: Option<String>,
    }

    // route handler
    pub async fn bind_routes() -> Router {
        let mut routes = Router::new();
        routes = routes.route("/search", get(search_song));
        routes = routes.route("/details", get(song_details));

        routes
    }

    // controllers

    async fn search_song(Query(query): Query<SearchSongQuery>) -> Json<Value> {
        let search_input = match query.search_input {
            Some(s) if !s.is_empty() => s,
            _ => {
                return Json(json!({
                    "status": "error",
                    "code": 400,
                    "message": "Missing or empty query parameter: search_input"
                }));
            }
        };

        let config = get_configs();
        let url = format!("{}{}", config.search_base_url, search_input);
        println!("{}", url);

        let client = reqwest::Client::new();

        // Make the request
        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Failed to make HTTP call: {}", err);
                return Json(json!({
                    "status": "error",
                    "code": 502,
                    "message": "Failed to make external request"
                }));
            }
        };

        // Check HTTP status
        if let Err(status_err) = response.error_for_status_ref() {
            eprintln!("Upstream returned error status: {}", status_err);
            return Json(json!({
                "status": "error",
                "code": response.status().as_u16(),
                "message": "Upstream service returned an error"
            }));
        }

        // Parse the JSON
        let body: Value = match response.json().await {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                return Json(json!({
                    "status": "error",
                    "code": 500,
                    "message": "Failed to parse response"
                }));
            }
        };

        // Return success
        Json(json!({
            "status": "ok",
            "code": 200,
            "data": body
        }))
    }

    async fn song_details(Query(query): Query<SongDetailsQuery>) -> Json<Value> {
        let id = match query.id {
            Some(s) if !s.is_empty() => s,
            _ => {
                return Json(json!({
                    "status": "error",
                    "code": 400,
                    "message": "Missing or empty query parameter: id"
                }));
            }
        };

        let config = get_configs();
        let client = reqwest::Client::new();
        let url = format!("{}{}", config.song_details_url, id);

        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Failed to make HTTP call: {}", err);
                return Json(json!({
                    "status": "error",
                    "code": 502,
                    "message": "Failed to make external request"
                }));
            }
        };

        if let Err(status_err) = response.error_for_status_ref() {
            eprintln!("Upstream returned error status: {}", status_err);
            return Json(json!({
                "status": "error",
                "code": response.status().as_u16(),
                "message": "Upstream service returned an error"
            }));
        }

        let body: Value = match response.json().await {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                return Json(json!({
                    "status": "error",
                    "code": 500,
                    "message": "Failed to parse response"
                }));
            }
        };

        Json(json!({
            "status": "ok",
            "code": 200,
            "data": body
        }))
    }
}
