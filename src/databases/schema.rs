table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        text -> Text,
    }
}

table! {
    memo_tag_rels (id) {
        id -> Int8,
        tag_id -> Int8,
        memo_id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    memos (id) {
        id -> Int8,
        title -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    micrate_db_version (id) {
        id -> Int4,
        version_id -> Int8,
        is_applied -> Bool,
        tstamp -> Nullable<Timestamp>,
    }
}

table! {
    post_tag_rels (id) {
        id -> Int4,
        tag_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    tag_groups (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        label -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    memo_tag_rels,
    memos,
    micrate_db_version,
    post_tag_rels,
    posts,
    tag_groups,
    tags,
);
