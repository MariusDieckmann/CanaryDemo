#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


use std::{sync::{Arc, Mutex, RwLock}};    
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::collections::VecDeque;
use rocket_contrib::json::{Json};
use rocket_contrib::serve::StaticFiles;
use rocket::{State};
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

    let load_generator = LoadGenerator::new();

    let _handle = thread::spawn(move || {
        read_stats(color_counter_wrapped_clone, api_key_value)
    });

    let err = rocket::ignite()
    .attach(Template::fairing())
    .manage(color_counter_wrapped.clone())
    .manage(api_key_value2)
    .manage(load_generator)
    .mount("/static", StaticFiles::from("static"))
    .mount("/", routes![index, stats, stats_show, start_generate_load, stop_generate_load])
    .launch();

    println!("{:?}", err)

}

/// returns a blank site with the background color set to the color returned by the backend
#[get("/")]
fn index(api_key: State<ApiKey>, load_generator: State<Arc<LoadGenerator>>) -> Template {
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

    let mut resp = client.get(&url).header("x-api-key", &api_key.0).send().unwrap()
        .json::<HashMap<String, String>>().unwrap();


    let load_status = &load_generator.generate_load.read().unwrap();

    resp.insert("generate_load".to_string(), std::format!("{}", load_status));


    println!("{:?}", resp);

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
    let context: HashMap<String, String> = HashMap::new();
    return Template::render("stats", context)
}


#[get("/load/start")]   
fn start_generate_load(load_generator: State<Arc<LoadGenerator>>) -> Redirect {
    load_generator.start_generate_load();

    return Redirect::to(uri!(index))
}

#[get("/load/stop")]   
fn stop_generate_load(load_generator: State<Arc<LoadGenerator>>) -> Redirect {
    load_generator.end_generate_load();

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

struct LoadGenerator {
    generate_load: RwLock<bool>
}

impl LoadGenerator {
    fn new() -> Arc<LoadGenerator> {

        let load_generator = Arc::new(LoadGenerator{
            generate_load: RwLock::new(false)
        });

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

        let load_generator_1 = load_generator.clone();

        thread::spawn(move || {
            loop {
                match *load_generator_1.generate_load.read().unwrap() {
                    true => {
                        match reqwest::blocking::get(&url) {
                            Ok(_) => {},
                            Err(_) => {},
                        }
                        thread::sleep(Duration::new(0, 500000))
                    },
                    false => {
                        thread::sleep(Duration::new(1, 0))
                    }
                }
            }
        });

        return load_generator
    }

    /// Starts load generation
    fn start_generate_load(&self) {
        *self.generate_load.write().unwrap() = true;
    }
 
    /// Ends load generation
    fn end_generate_load(&self) {
        *self.generate_load.write().unwrap() = false
    }
}

#[cfg(test)]
mod tests {
    use crate::LoadGenerator;

    #[test]
    fn load_generator_test() {
        let load_generator = LoadGenerator::new();

        load_generator.start_generate_load()

    }
}