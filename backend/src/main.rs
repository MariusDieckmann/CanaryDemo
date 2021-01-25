#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;

use std::env;
use std::{fs::File};
use std::thread;

use rocket_contrib::json::{Json};
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;
use rocket::State;

use clap::Clap;

const COLORCODE: &str = "green";

struct ApiKeyStruct(String);
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

    let path = opts.config;

    println!("Opening file: {}", path);

    let file = File::open(path)?;
    let config: Config = serde_yaml::from_reader(file).expect("Could not parse config file");
    
    let api_key = "APIKey";
    let api_key_value: ApiKey = ApiKey(env::var(api_key).expect("Could not read env var for secret key"));
    
    rocket::ignite().manage(config).manage(api_key_value).mount("/", routes![get_color, get_health, get_load]).launch();

    return Ok(());
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyStruct {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let api_key = match request.guard::<State<ApiKey>>() {
            Outcome::Failure(_) => return Outcome::Failure((Status::InternalServerError, ApiKeyError::BadCount)),
            Outcome::Success(s) => s,
            Outcome::Forward(_) => return Outcome::Failure((Status::InternalServerError, ApiKeyError::BadCount)),
        };

        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0], api_key) => Outcome::Success(ApiKeyStruct(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str, original_key: State<ApiKey>) -> bool {
    key == original_key.0
}


// Returns the defined color
#[get("/color")]
fn get_color(_api: ApiKeyStruct) -> Status {
    return Status::InternalServerError;
}

#[get("/health")]
fn get_health(config: State<Config>) -> Status {
    if config.should_fail {
        return Status::InternalServerError
    };

    return Status::Ok
}

#[get("/load")]
fn get_load() -> Status {
    
    thread::spawn(||{
        let mut i: i64 = 0;

        for _ in 0..1000000000 {
            i = i + 1;
        }
    });

    return Status::Ok
}