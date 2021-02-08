#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;

use std::{env, time::{Instant}};
use std::{fs::File};
use std::thread;

use std::path::Path;

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
    load_iterations: i64,
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

    let path_string = opts.config;

    println!("Opening file: {}", path_string);

    let path = Path::new(&path_string);

    let mut config = Config{
        should_fail: false,
        load_iterations: 10000
    };

    if path.is_file() {
        let file = File::open(path)?;
        config = serde_yaml::from_reader(file).expect("Could not parse config file");
    }
    
    let start_time = StartTime{
        time_started: Instant::now(),
    };

    let api_key = "APIKey";
    let api_key_value: ApiKey = ApiKey(env::var(api_key).expect("Could not read env var for secret key"));
    
    rocket::ignite().manage(config).manage(start_time).manage(api_key_value).mount("/", routes![get_color, get_health_live, get_health_startup, get_load]).launch();

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
fn get_color(_api: ApiKeyStruct) -> Json<Color> {
    let color_code = Color{color: COLORCODE.to_string()};

   Json(color_code)
}

#[get("/health/live")]
fn get_health_live(config: State<Config>) -> Status {
    if config.should_fail {
        return Status::InternalServerError
    };

    return Status::Ok
}

#[get("/health/startup")]
fn get_health_startup(start_time: State<StartTime>) -> Status {
    let duration = start_time.time_started.elapsed();

    if duration.as_secs() < 5 {
        return Status::ServiceUnavailable
    }

    return Status::Ok
}

#[get("/load")]
fn get_load(conf: State<Config>) -> Status {
    
    let load_iterations = conf.load_iterations;

    thread::spawn(move ||{
        let mut primes:Vec<i64> = Vec::new();

        for i in 2i64..load_iterations {
            let mut is_prime = true;
            for j in 2..i-1 {
                if i%j == 0 {
                    is_prime = false;
                }
            }

            if is_prime {
                primes.push(i);
            }
        }
    });

    return Status::Ok
}

struct StartTime {
    time_started: Instant,
}