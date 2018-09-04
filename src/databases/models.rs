extern crate diesel;

use super::schema::posts;
use super::schema::comments;

use diesel::*;

#[derive(Queryable, Identifiable, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


// この辺を Post <-> Commentでderiveしてないとダメ
#[derive(Queryable, Associations, PartialEq, Identifiable)]
#[belongs_to(Post, foreign_key = "post_id")]
#[table_name = "comments"]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub text: String,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComments<'a> {
    pub post_id: &'a i32,
    pub text: &'a str,
}