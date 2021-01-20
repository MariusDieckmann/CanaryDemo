#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;

use std::{fs::File};

use rocket_contrib::json::{Json};
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;
use rocket::State;

use clap::Clap;

const COLORCODE: &str = "green";

struct ApiKey(String);

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    should_fail: bool,
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Marius D. <Marius.Dieckmann@computational.bio.uni-giessen.de>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "config/config.yaml")]
    config: String,
}

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let file = File::open(opts.config)?;
    let config: Config = serde_yaml::from_reader(file).expect("Could not parse config file");
    
    println!("{}", config.should_fail);

    rocket::ignite().manage(config).mount("/", routes![get_color, get_health]).launch();

    return Ok(());
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "valid_api_key"
}

// Returns the defined color
#[get("/color")]
fn get_color() -> Json<Color> {
    let color_code = Color{color: COLORCODE.to_string()};

   Json(color_code)
}

#[get("/health")]
fn get_health(config: State<Config>) -> Status {
    if config.should_fail {
        return Status::InternalServerError
    };

    return Status::Ok
}