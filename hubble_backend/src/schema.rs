table! {
    matches (id) {
        id -> Varchar,
        player_id -> Varchar,
        opening_id -> Varchar,
        moves -> Jsonb,
        scores -> Jsonb,
        winner -> Varchar,
        player_rating -> Nullable<Int4>,
        oponnent_rating -> Nullable<Int4>,
        is_white -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        rating -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    matches,
    users,
);
