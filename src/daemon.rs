extern crate failure;
extern crate gotham;
extern crate gtfs_structures;
extern crate mime;

use hyper::{Response, StatusCode};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::http::response::create_response;
use validators::validate;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    url: String,
}

fn validation_handler(mut state: State) -> (State, Response) {
    let query_param = QueryStringExtractor::take_from(&mut state);

    let res = match validate(&query_param.url) {
        Ok(json) => create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        ),
        Err(err) => create_response(
            &state,
            StatusCode::InternalServerError,
            Some((
                format!("{{\"error\": \"{}\"}}", err).into_bytes(),
                mime::APPLICATION_JSON,
            )),
        ),
    };

    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/validate")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(validation_handler);
    })
}

pub fn run_server() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}