extern crate rocket;
extern crate diesel;
extern crate serde_json;

use databases::connection;
use databases::schema::posts::dsl::*;
use databases::schema::posts;
use databases::models::{ Post, NewPost };
use diesel::prelude::*;
use rocket_contrib::json::Json;


#[get("/posts/all")]
fn all() -> String {
    let pg_conn = connection::establish_connection();
    let results: Vec<String> = posts
        .load::<Post>(&pg_conn)
        .expect("Error loading posts")
        .into_iter().rev().map(|post: Post| post.body).collect();

    format!("{:?}", results)
}

#[derive(Serialize, Deserialize)]
struct PostArg {
    title: String,
    body: String
}

#[post("/posts", data = "<post_arg>", format = "application/json")]
fn post(post_arg: Json<PostArg>) -> String {
    use std::time::Instant;

    let now = Instant::now().clone().elapsed();
    let pg_conn = connection::establish_connection();
    let new_post = NewPost {
        title: &post_arg.title.to_string(),
        body: &post_arg.body.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&pg_conn)
        .expect("Error saving new post");

    format!("{:?}", now)
}

