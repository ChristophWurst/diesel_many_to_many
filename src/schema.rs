// Generated with
// $ diesel print-schema

table! {
    images (id) {
        id -> Int4,
        url -> Varchar,
    }
}

table! {
    image_tags (id) {
        id -> Int4,
        image_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        label -> Varchar,
    }
}

joinable!(image_tags -> images (image_id));
joinable!(image_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    images,
    image_tags,
    tags,
);

