table! {
    request (id) {
        id -> Int4,
        domain -> Varchar,
        request_date -> Date,
        response -> Json,
        throttled -> Bool,
    }
}
