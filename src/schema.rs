table! {
    bans (id) {
        id -> Int4,
        admin_user_id -> Int4,
        banned_user_id -> Int4,
        guild_id -> Int8,
        reason -> Varchar,
        ban_time -> Int8,
        end_time -> Int8,
        lifted -> Int8,
    }
}

table! {
    inventory (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        amount -> Int4,
    }
}

table! {
    shop (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        price -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        discord_id -> Int8,
        messages -> Int4,
        money -> Int4,
        daily_claimed -> Int8,
        bot_admin -> Bool,
    }
}

table! {
    warnings (id) {
        id -> Int4,
        admin_user_id -> Int4,
        warned_user_id -> Int4,
        guild_id -> Int8,
        reason -> Varchar,
        warning_time -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    bans,
    inventory,
    shop,
    users,
    warnings,
);
