// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        path -> Text,
        name -> Nullable<Text>,
        author -> Nullable<Text>,
        artist -> Nullable<Text>,
        description -> Nullable<Text>,
        notes -> Nullable<Text>,
        created -> Nullable<BigInt>,
        is_exif -> Nullable<Bool>,
        gps_altitude -> Nullable<Integer>,
        gps_latitude -> Nullable<Integer>,
        x_res -> Nullable<Integer>,
        y_res -> Nullable<Integer>,
        deleted -> Bool,
        updated_at -> Nullable<BigInt>,
    }
}

diesel::table! {
    tag_file (id) {
        id -> Integer,
        label_id -> Integer,
        tag_id -> Integer,
        created_at -> Nullable<BigInt>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        parent -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<BigInt>,
        updated_at -> Nullable<BigInt>,
    }
}

diesel::joinable!(tag_file -> files (label_id));
diesel::joinable!(tag_file -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    tag_file,
    tags,
);
