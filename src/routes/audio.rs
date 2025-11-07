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
        search_input: String,
    }

    #[derive(Deserialize, Debug)]
    struct SongDetailsQuery {
        id: String,
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
        let config = get_configs();
        println!(
            "{}",
            format!("{}{}", config.search_base_url, query.search_input)
        );

        let client = reqwest::Client::new();

        // Make the request
        let response = match client
            .get(format!("{}{}", config.search_base_url, query.search_input))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Failed to make HTTP call: {}", err);
                return Json(json!({
                    "status": "error",
                    "code": 500,
                    "message": "Failed to make external request"
                }));
            }
        };

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
        let config = get_configs();
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", config.song_details_url, query.id))
            .send()
            .await
            .unwrap();
        let body: Value = response.json().await.unwrap();

        Json(json!({
            "status": "ok",
            "code": 200,
            "data": body
        }))
    }
}
