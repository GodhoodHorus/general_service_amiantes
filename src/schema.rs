table! {
    authorizations (id) {
        id -> Int4,
        level -> Varchar,
    }
}

table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        address -> Jsonb,
        interlocutors -> Nullable<Jsonb>,
        created_at -> Timestamp,
        edited_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        authorization_id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

table! {
    worksites (id) {
        id -> Int4,
        client_id -> Int4,
        worksite -> Jsonb,
        created_at -> Timestamp,
        edited_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(users -> authorizations (authorization_id));
joinable!(worksites -> clients (client_id));

allow_tables_to_appear_in_same_query!(
    authorizations,
    clients,
    users,
    worksites,
);
