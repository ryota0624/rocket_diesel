extern crate rocket;

use rocket::http::RawStr;

#[get("/msg/<msg>")]
fn handler(msg: &RawStr) -> String {
    msg.as_str().to_string()
}

impl MsgController for COntroller {
    fn handlers() {
        vec![
            handler,
            handler
            ]
    }
}