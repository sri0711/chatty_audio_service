pub mod audio_routes {

    // imports
    use crate::{
        constants::audio_constant::{SearchSongMain, SearchSongQuery, SongDetailsQuery},
        helper::config::configurations::get_configs,
    };
    use axum::{extract::Query, http::HeaderMap, routing::get, Json, Router};
    use reqwest::{self, header::COOKIE};
    use serde_json::{json, Value};

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

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.append(
            COOKIE,
            format!("L=english; gdpr_acceptance=true; DL=english")
                .parse()
                .unwrap(),
        );
        // Make the request
        let response = match client.get(&url).headers(headers).send().await {
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
        print!("got response");
        // Parse the JSON
        let mut response_body: SearchSongMain = match response.json().await {
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

        for song in &mut response_body.results.iter_mut() {
            for data in &mut song.iter_mut() {
                for info_data in &mut data.more_info.iter_mut() {
                    if info_data.is_320.as_deref() == Some("true") {
                        info_data.high_quality_link = Some(
                            info_data
                                .encrypted_media_url
                                .clone()
                                .expect("")
                                .replace("_96", "_320")
                                .to_owned(),
                        );
                    } else {
                        info_data.high_quality_link = Some(
                            info_data
                                .encrypted_media_url
                                .clone()
                                .expect("")
                                .replace("_96", "_160")
                                .to_owned(),
                        );
                    }
                }
            }
        }

        // Return success
        Json(json!({
            "status": "ok",
            "code": 200,
            "data": response_body
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
