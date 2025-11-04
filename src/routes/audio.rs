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

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", config.search_base_url, query.search_input))
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
