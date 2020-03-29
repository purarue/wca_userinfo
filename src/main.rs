use std::env;

use iron;
use iron::prelude::*;
use iron::status;
use router::Router;

use reqwest;
use serde_json;

mod model;
mod parse;

static APP_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0";
static BASE_WCA_URL: &str = "https://www.worldcubeassociation.org/persons";
static DEFAULT_PORT: u64 = 8010;

/// get port from environment variable or use default
fn get_port() -> u64 {
    let port_envvar = "WCA_USERINFO_PORT";
    match env::var(port_envvar) {
        Ok(env_port) => match env_port.parse::<u64>() {
            Ok(port_num) => port_num,
            Err(_) => {
                eprintln!(
                    "Failed to parse '{}' as u64, using default: '{}'",
                    env_port, DEFAULT_PORT
                );
                DEFAULT_PORT
            }
        },
        Err(_) => DEFAULT_PORT,
    }
}

/// controller for all /wca_id endpoint
fn controller(req: &mut Request) -> IronResult<Response> {
    // request::get makes client implicitly, this creates it explicitly
    // so that we set the user agent.
    // could used a arc/mutex to share across requests, but the performance
    // impact is negligible
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let query = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("wca_id")
        .unwrap_or("/");
    match parse::get_page_contents(&client, &format!("{}/{}", BASE_WCA_URL, query)) {
        Ok(request_body) => match &parse::parse_html(&request_body) {
            // request succeeded, parsing succeeded
            Ok(user_info) => Ok(Response::with((
                status::Ok,
                serde_json::to_string(user_info).unwrap(),
            ))),
            // request succeeded, parsing failed
            Err(parse_err) => Ok(Response::with((
                status::BadRequest,
                serde_json::to_string(&serde_json::json!({ "error": format!("{}", parse_err) }))
                    .unwrap(),
            ))),
        },
        // request failed
        Err(req_err) => Ok(Response::with((
            status::BadRequest,
            serde_json::to_string(&serde_json::json!({ "error": format!("{}", req_err) })).unwrap(),
        ))),
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/:wca_id", controller, "wca_id");

    let port = get_port();

    let _app = Iron::new(router).http(format!("0.0.0.0:{}", port)).unwrap();
    println!("Hosting wca_userinfo server on port: {}", port);
}

//  ispell
//  LocalWords:  reqwest serde json str WCA fn req mut mutex wca Ok http
//  LocalWords:  println
