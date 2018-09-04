-- Your SQL goes here

CREATE TABLE comments(
  id serial primary key ,
  post_id serial not null ,
  text text not null
);