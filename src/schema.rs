table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        iscompleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
