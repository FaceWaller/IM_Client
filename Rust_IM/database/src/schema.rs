use diesel::*;
table! {
    im_table (autoId) {
        autoId -> BigInt,
        fromId -> Text,
        toId -> Text,
        time -> BigInt,
        format -> Integer,
        sid -> Text,
        msg -> Text,
    }
}

allow_tables_to_appear_in_same_query!(im_table,);
