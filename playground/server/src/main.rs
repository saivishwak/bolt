use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Json, Router,
};
use bolt::{error::BoltError, evaluator::evaluator::Evaluator};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Template, Debug)]
#[template(path = "playground.html")]
struct PlaygroundTemplate {}

#[derive(Template, Debug)]
#[template(path = "results.html")]
struct ResultsTemplate {
    result: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RunPayload {
    code: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

async fn playground() -> Html<&'static str> {
    let playground = PlaygroundTemplate {};
    Html(string_to_static_str(playground.render().unwrap()))
}

#[allow(unused_assignments)]
async fn run(Json(payload): Json<RunPayload>) -> Html<&'static str> {
    let code = payload.code.clone();
    let mut evaluated_result = String::new();
    let evaluator = Evaluator::new(code, None, false, None);
    match evaluator.eval() {
        Some(evaluated) => match evaluated {
            Ok(result) => {
                evaluated_result = result.inspect();
            }
            Err(e) => {
                evaluated_result = e.get_message();
            }
        },
        None => {
            evaluated_result = "Something went wrong!".to_string();
        }
    }
    let results = ResultsTemplate {
        result: evaluated_result,
    };
    Html(string_to_static_str(results.render().unwrap()))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(playground))
        .route("/run", post(run))
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
