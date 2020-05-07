table! {
    bookmarks (id) {
        id -> Uuid,
        title -> Text,
        url -> Text,
        icon -> Text,
        notes -> Text,
        relevant -> Timestamp,
        created -> Timestamp,
    }
}

table! {
    bookmark_tags (id) {
        id -> Int4,
        bookmark_id -> Uuid,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        color -> Text,
    }
}

table! {
    units (id) {
        id -> Int4,
        name -> Text,
        building -> Text,
        minerals -> Int4,
        gas -> Int4,
        supply -> Int4,
    }
}

joinable!(bookmark_tags -> bookmarks (bookmark_id));
joinable!(bookmark_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(bookmarks, bookmark_tags, tags, units,);
