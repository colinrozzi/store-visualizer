mod bindings;

use bindings::exports::ntwk::theater::actor::Guest as ActorGuest;
use bindings::exports::ntwk::theater::http_server::Guest as HttpGuest;
use bindings::exports::ntwk::theater::http_server::{
    HttpRequest as ServerHttpRequest, HttpResponse,
};
use bindings::exports::ntwk::theater::message_server_client::Guest as MessageServerClientGuest;
use bindings::ntwk::theater::filesystem::read_file;
use bindings::ntwk::theater::message_server_host::request;
use bindings::ntwk::theater::runtime::log;
use bindings::ntwk::theater::types::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct State {
    store_id: String,
}

// Reuse the Request/Action types from key-value actor
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    _type: String,
    data: Action,
}

#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Get(String),
    Put(Vec<u8>),
    All(()),
}

impl State {
    fn get_all_entries(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let req = Request {
            _type: "request".to_string(),
            data: Action::All(()),
        };

        let request_bytes = serde_json::to_vec(&req)?;
        let response_bytes = request(&self.store_id, &request_bytes)?;

        let response: Value = serde_json::from_slice(&response_bytes)?;
        if response["status"].as_str() == Some("ok") {
            Ok(response["data"].clone())
        } else {
            Err("Failed to get store entries".into())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InitData {
    store_id: String,
}

struct Component;

impl ActorGuest for Component {
    fn init(data: Option<Vec<u8>>) -> Vec<u8> {
        log("Initializing store visualizer actor");
        let data = data.unwrap();

        let init_data: InitData = serde_json::from_slice(&data).unwrap();
        log(&format!("Store actor id: {}", init_data.store_id));

        let initial_state = State {
            store_id: init_data.store_id,
        };

        serde_json::to_vec(&initial_state).unwrap()
    }
}

impl HttpGuest for Component {
    fn handle_request(req: ServerHttpRequest, state: Json) -> (HttpResponse, Json) {
        log(&format!("Handling HTTP request for: {}", req.uri));

        match (req.method.as_str(), req.uri.as_str()) {
            ("GET", "/") | ("GET", "/index.html") => {
                let content = read_file("index.html").unwrap();
                (
                    HttpResponse {
                        status: 200,
                        headers: vec![("Content-Type".to_string(), "text/html".to_string())],
                        body: Some(content),
                    },
                    state,
                )
            }
            ("GET", "/styles.css") => {
                let content = read_file("styles.css").unwrap();
                (
                    HttpResponse {
                        status: 200,
                        headers: vec![("Content-Type".to_string(), "text/css".to_string())],
                        body: Some(content),
                    },
                    state,
                )
            }
            ("GET", "/visualizer.js") => {
                let content = read_file("visualizer.js").unwrap();
                (
                    HttpResponse {
                        status: 200,
                        headers: vec![(
                            "Content-Type".to_string(),
                            "application/javascript".to_string(),
                        )],
                        body: Some(content),
                    },
                    state,
                )
            }
            ("GET", "/api/store-contents") => {
                let current_state: State = serde_json::from_slice(&state).unwrap();
                match current_state.get_all_entries() {
                    Ok(entries) => (
                        HttpResponse {
                            status: 200,
                            headers: vec![(
                                "Content-Type".to_string(),
                                "application/json".to_string(),
                            )],
                            body: Some(
                                serde_json::to_vec(&json!({
                                    "status": "success",
                                    "entries": entries
                                }))
                                .unwrap(),
                            ),
                        },
                        state,
                    ),
                    Err(_) => (
                        HttpResponse {
                            status: 500,
                            headers: vec![],
                            body: Some(b"Failed to load store contents".to_vec()),
                        },
                        state,
                    ),
                }
            }
            // Default 404 response
            _ => (
                HttpResponse {
                    status: 404,
                    headers: vec![],
                    body: Some(b"Not Found".to_vec()),
                },
                state,
            ),
        }
    }
}

impl MessageServerClientGuest for Component {
    fn handle_send(msg: Json, state: Json) -> Json {
        log("Handling message server client send");
        log(&format!(
            "Message: {}",
            String::from_utf8(msg).unwrap_or_default()
        ));
        state
    }

    fn handle_request(msg: Json, state: Json) -> (Json, Json) {
        log("Handling message server client request");
        log(&format!(
            "Message: {}",
            String::from_utf8(msg).unwrap_or_default()
        ));
        (vec![], state)
    }
}

bindings::export!(Component with_types_in bindings);
