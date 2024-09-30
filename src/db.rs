diesel::table! {
    files {
        id -> Integer,
        path -> Text,
        name -> Nullable<Text>,
        author -> Nullable<Text>,
        artist -> Nullable<Text>,
        description -> Nullable<Text>,
        notes -> Nullable<Text>,
        fs_created -> Integer,
        exif_created -> Nullable<Integer>,
        gps_altitude -> Nullable<Integer>,
        gps_latitude  -> Nullable<Integer>,
        x_res -> Nullable<Integer>,
        y_res -> Nullable<Integer>,
        deleted -> Bool
    }
}

diesel::table! {
    tags {
        id -> Integer,
        parent -> Integer,
        name -> Text,
        description -> Nullable<Text>
    }
}

diesel::table! {
    tag_file {
        id -> Integer,
        label_id -> Integer,
        tag_id -> Integer,
    }
}
