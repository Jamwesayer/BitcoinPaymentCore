table! {
    payment_status (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    payment_window (id) {
        id -> Integer,
        label -> Varchar,
        amount -> Double,
        date -> Date,
        payment_status_id -> Integer,
        store_id -> Integer,
    }
}

table! {
    shop (id) {
        id -> Integer,
        name -> Text,
        address -> Varchar,
        wallet_address -> Varchar,
    }
}

table! {
    transaction (id) {
        id -> Integer,
        amount -> Double,
        hash -> Varchar,
        from_address -> Varchar,
        date -> Datetime,
        transaction_type_id -> Integer,
        transaction_status_id -> Integer,
        payment_window_id -> Integer,
    }
}

table! {
    transaction_status (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    transaction_type (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    payment_status,
    payment_window,
    shop,
    transaction,
    transaction_status,
    transaction_type,
);

joinable!(transaction -> payment_window (payment_window_id));
