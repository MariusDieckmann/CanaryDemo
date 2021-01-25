#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


use std::sync::{Arc, Mutex};    
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::collections::VecDeque;
use rocket_contrib::json::{Json};
use rocket_contrib::serve::StaticFiles;
use rocket::{State, http::Status};
use rocket::response::Redirect;
use std::env;

extern crate serde;
extern crate serde_json;

struct ApiKey(String);

use std::thread;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

const BLUESTRING: &str = "blue";
const GREENSTRING: &str = "green";


fn main() {
    let color_counter = ColorCount {
        blue: VecDeque::new(),
        green: VecDeque::new(),
        error: VecDeque::new(),
        timestamps: VecDeque::new(),        
    };

    let color_counter_wrapped = Arc::new(Mutex::new(color_counter));

    let color_counter_wrapped_clone = color_counter_wrapped.clone();

    let api_key = "APIKey";
    let api_key_value: ApiKey = ApiKey(env::var(api_key).expect("Could not read env var for secret key"));
    let api_key_value2: ApiKey = ApiKey(env::var(api_key).expect("Could not read env var for secret key"));

    let _handle = thread::spawn(move || {
        read_stats(color_counter_wrapped_clone, api_key_value)
    });

    let err = rocket::ignite()
    .attach(Template::fairing())
    .manage(color_counter_wrapped.clone())
    .manage(api_key_value2)
    .mount("/static", StaticFiles::from("static"))
    .mount("/", routes![index, stats, stats_show, generate_load])
    .launch();

    println!("{:?}", err)

}

/// returns a blank site with the background color set to the color returned by the backend
#[get("/")]
fn index(api_key: State<ApiKey>) -> Template {
    let backend_host = match std::env::var("backend_host") {
        Ok(resp) => resp,
        Err(e) => {
            println!("{}", e);
            "localhost".to_string()
        },
    };
    let backend_port = match std::env::var("backend_port") {
        Ok(resp) => resp,
        Err(e) => {
            println!("{}", e);
            "8001".to_string()
        },
    };
    

    let url = format!("http://{}:{}/color", backend_host, backend_port);

    let client = reqwest::blocking::Client::new();

    let resp = client.get(&url).header("x-api-key", &api_key.0).send().unwrap()
        .json::<HashMap<String, String>>().unwrap();
    return Template::render("base", &resp);
}

/// Returns the current number of colors found from the backend based on the
/// read_stats method
#[get("/stats/data")]
fn stats(color_count: State<Arc<Mutex<ColorCount>>>) -> Json<ColorCount> {
    let color_counter_guard = color_count.lock().unwrap();
    let stats = color_counter_guard.get_stats();

    return stats;
}


/// Returns a visual representation of the backend get color results
/// as read from the read_stats method
#[get("/stats/show")]   
fn stats_show() -> Template {
    let foo: HashMap<String, String> = HashMap::new();
    return Template::render("stats", foo)
}

#[get("/load")]   
fn generate_load() -> Redirect {
    let backend_host = match std::env::var("backend_host") {
        Ok(resp) => resp,
        Err(_) => {
            "localhost".to_string()
        },
    };
    let backend_port = match std::env::var("backend_port") {
        Ok(resp) => resp,
        Err(_) => {
            "8001".to_string()
        },
    };

    let url = format!("http://{}:{}/load", backend_host, backend_port);

    thread::spawn(move ||{
        reqwest::blocking::get(&url)
    });


    return Redirect::to(uri!(index))

}

/// A continues loop that reads the number of hits of green and blue responses from
/// the backend. The result is stored in a queue. Only the most recent 100 hits are
/// stored
fn read_stats<'a>(color_counter_wrapper: Arc<Mutex<ColorCount>>, api_key: ApiKey) -> () {
    loop {
        let now = SystemTime::now();
        let time_since_epoch = now.duration_since(UNIX_EPOCH).expect("Failed to get time");
        let mut color_blue = 0i64;
        let mut color_green = 0i64;
        let mut error_counter = 0i64;

        let backend_host = match std::env::var("backend_host") {
            Ok(resp) => resp,
            Err(_) => {
                "localhost".to_string()
            },
        };
        let backend_port = match std::env::var("backend_port") {
            Ok(resp) => resp,
            Err(_) => {
                "8001".to_string()
            },
        };
        
    
        let url = format!("http://{}:{}/color", backend_host, backend_port);

        let client = reqwest::blocking::Client::new();

        for _ in 0..100 {
            let resp_result = client.get(&url).header("x-api-key", &api_key.0).send();

            let resp = match resp_result {
                Ok(resp) => resp,
                Err(e) => {
                    println!("{}", e);
                    error_counter = error_counter + 1i64;
                    continue;
                },
            };
            let json_resp_result = resp.json::<HashMap<String, String>>();
            let json_resp = match json_resp_result {
                Ok(json_resp) => json_resp,
                Err(e) => {
                    println!("{}", e);
                    error_counter = error_counter + 1i64;
                    continue;
                },
            };
            let color_results = json_resp.get("color");
            let color = match color_results {
                Some(color_data) => color_data.as_str(),
                None => {
                    error_counter = error_counter + 1i64;
                    continue;
                },
            };
            match color {
                BLUESTRING => color_blue = color_blue + 1i64,
                GREENSTRING => color_green = color_green + 1i64,
                _ => (),
            }
        }

        {
            let mut color_counter_guard = color_counter_wrapper.lock().unwrap();
            color_counter_guard.blue.push_front(color_blue);
            color_counter_guard.green.push_front(color_green);
            color_counter_guard.timestamps.push_front(time_since_epoch.as_secs());

            if color_counter_guard.blue.len() > 100 {
                color_counter_guard.blue.pop_back();
            }

            if color_counter_guard.green.len() > 100 {
                color_counter_guard.green.pop_back();
            }

            if color_counter_guard.timestamps.len() > 100 {
                color_counter_guard.timestamps.pop_back();
            }

        }

        thread::sleep(Duration::from_millis(1000));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorCount {
    blue: VecDeque<i64>,
    green: VecDeque<i64>,
    error: VecDeque<i64>,
    timestamps: VecDeque<u64>,
}

impl ColorCount {
    fn get_stats (&self) -> Json<ColorCount> {
        return Json(self.clone())
    }
}