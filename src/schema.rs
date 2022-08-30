table! {
    frames (id) {
        id -> Text,
        start -> Timestamp,
        end -> Nullable<Timestamp>,
        last_update -> Timestamp,
        project -> Text,
        tags -> Text,
        deleted -> Bool,
    }
}
