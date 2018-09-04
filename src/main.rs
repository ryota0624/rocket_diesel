#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;

use rocket::request::Request;

mod controllers;

use controllers::*;

mod databases;

#[get("/")]
fn index() -> String {
    "H".to_string()
}

#[catch(404)]
fn not_found(_req_: &Request) -> String {
    "not_found".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            msg_controller::handler,
            post_controller::all,
            post_controller::post
            ])
        .catch(catchers![not_found])
        .launch();
}
