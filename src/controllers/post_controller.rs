extern crate rocket;
extern crate diesel;
extern crate serde_json;

use databases::connection;
use databases::schema::posts::dsl::*;
use databases::schema::posts;

use databases::models::{Post, NewPost, Comment};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use controller::*;

#[derive(Serialize, Deserialize)]
struct PostJson {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct CommentJson {
    text: String
}

#[derive(Serialize, Deserialize)]
struct PostWithComments {
    post: PostJson,
    comments: Vec<CommentJson>
}

#[get("/posts/all")]
fn all() -> Json<JsonResult<Vec<PostWithComments>>> {
    let pg_conn = connection::establish_connection();
    let post_daos: Vec<Post> = posts
        .load::<Post>(&pg_conn)
        .expect("Error loading posts")
        .into_iter().rev().collect();

    let results =
            post_daos
                .into_iter()
                .map (|post_dao: Post| -> PostWithComments {
                    let comments = Comment::belonging_to(&post_dao)
                        .load::<Comment>(&pg_conn)
                        .expect("Error loading comments");
                    PostWithComments {
                        post: post_dao_to_json(post_dao),
                        comments: comments.into_iter().map(comment_dao_to_json).collect::<Vec<CommentJson>>()
                    }
                }).collect::<Vec<PostWithComments>>();



    json_response(results)
}


#[post("/posts", data = "<post_json>", format = "application/json")]
fn post(post_json: Json<PostJson>) -> String {
    use std::time::Instant;

    let now = Instant::now().clone().elapsed();
    let pg_conn = connection::establish_connection();
    let new_post = NewPost {
        title: &post_json.title.to_string(),
        body: &post_json.body.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&pg_conn)
        .expect("Error saving new post");

    format!("{:?}", now)
}

fn post_dao_to_json(dao: Post) -> PostJson {
    PostJson {
        title: dao.title,
        body: dao.body,
    }
}

fn comment_dao_to_json(dao: Comment) -> CommentJson {
    CommentJson {
        text: dao.text
    }
}
