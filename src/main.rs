use std::env;
use std::process::exit;

use chrono::Utc;
use iron::prelude::*;
use iron::status;
use router::Router;

mod lib;

static APP_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0";
static BASE_WCA_URL: &str = "https://www.worldcubeassociation.org/persons";
static DEFAULT_PORT: u64 = 8010;
static PORT_ENV_NAME: &str = "WCA_USERINFO_PORT";

/// get port from environment variable or use default
fn get_port() -> u64 {
    match env::var(PORT_ENV_NAME) {
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

struct WcaResponse {
    json_string: String,
    succeeded: bool,
}

/// Request data for this user and serialize it to JSON
fn request_json_data(wca_id: &str) -> WcaResponse {
    // request::get makes client implicitly, this creates it explicitly
    // so that we set the user agent.
    // could used a arc/mutex to share across requests, but the performance
    // impact is negligible
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    match lib::get_page_contents(&client, &format!("{}/{}", BASE_WCA_URL, wca_id)) {
        Ok(request_body) => match &lib::parse_html(&request_body) {
            Ok(user_info) => WcaResponse {
                json_string: serde_json::to_string(user_info).unwrap(),
                succeeded: true,
            },
            // request succeeded, parsing failed
            Err(parse_err) => WcaResponse {
                json_string: serde_json::to_string(&serde_json::json!({
                    "error": format!("{}", parse_err)
                }))
                .unwrap(),
                succeeded: false,
            },
        },
        // request failed
        Err(req_err) => WcaResponse {
            json_string: serde_json::to_string(&serde_json::json!({
                "error": format!("{}", req_err)
            }))
            .unwrap(),
            succeeded: false,
        },
    }
}

/// controller for all /wca_id endpoint
fn controller(req: &mut Request) -> IronResult<Response> {
    let query = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("wca_id")
        .unwrap_or("/");
    println!("[{}] GET /{}", Utc::now().to_rfc3339(), query);
    let res = request_json_data(query);
    let status = match res.succeeded {
        true => status::Ok,
        false => status::BadRequest,
    };
    Ok(Response::with((status, res.json_string)))
}

fn server() {
    let mut router = Router::new();
    router.get("/:wca_id", controller, "wca_id");

    let port = get_port();

    let _server = Iron::new(router).http(format!("0.0.0.0:{}", port)).unwrap();
    println!("Hosting wca_userinfo server on port: {}", port);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        1 => match args[0].as_str() {
            "server" => server(),
            wca_id => {
                let res = request_json_data(wca_id);
                println!("{}", res.json_string);
                if !res.succeeded {
                    exit(1)
                }
            }
        },
        _ => {
            eprintln!("Wrong number of arguments. Provide one -- either 'server' or a WCA User ID to get data for");
            exit(1);
        }
    }
}

//  ispell
//  LocalWords:  reqwest serde json str WCA fn req mut mutex wca Ok http
//  LocalWords:  println
