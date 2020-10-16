#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json};
use rocket::http::Status;

// blue
//const COLORCODE: &str = "blue";

// green
const COLORCODE:&str = "green";

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    color: String,
}


fn main() {
    rocket::ignite().mount("/", routes![get_color,get_healthz]).launch();
}


#[get("/healthz")]
fn get_healthz() -> Status {
    Status::Ok
}

/// Returns the defined color
#[get("/color")]
fn get_color() -> Json<Color> {
    let color_code = Color{color: COLORCODE.to_string()};

    Json(color_code)
}

