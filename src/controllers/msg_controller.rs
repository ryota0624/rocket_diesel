extern crate rocket;

use rocket::http::RawStr;

#[get("/msg/<msg>")]
fn handler(msg: &RawStr) -> String {
    msg.as_str().to_string()
}

