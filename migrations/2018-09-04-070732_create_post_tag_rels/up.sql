-- Your SQL goes here

CREATE TABLE post_tag_rels (
  id serial PRIMARY KEY,
  tag_id serial NOT NULL,
  post_id serial NOT NULL
)