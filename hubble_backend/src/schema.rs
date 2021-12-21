table! {
    games (id) {
        id -> Varchar,
        opening_id -> Nullable<Varchar>,
        moves -> Jsonb,
        scores -> Jsonb,
        white -> Varchar,
        black -> Varchar,
        white_rating -> Nullable<Int4>,
        black_rating -> Nullable<Int4>,
        winner -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        rating -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
