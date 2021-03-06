use crate::models::economy::PurchasableItem;
use crate::models::economy::*;
use crate::models::user::User;
use crate::DatabaseContainer;
use diesel::prelude::*;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(daily, buy, balance, set_money, reset_daily, new_item, shop, inventory)]
struct Economy;

#[command]
#[help_available]
#[num_args(0)]
#[only_in("guild")]
#[aliases("d", "day", "bread")]
#[description("A command to get your daily cash.")]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let discord_uid = msg.author.id.0 as i64;
        let mut user = User::get(discord_uid, db);
        match user.claim_daily(db) {
            Ok(_) => {
                msg.channel_id
                    .send_message(&ctx.http, |builder| {
                        builder
                            .reference_message(msg)
                            .allowed_mentions(|f| f.replied_user(true));
                        builder.embed(|e| {
                            e.colour(0xff0069).title("Daily coins claimed!").field(
                                "New Balance",
                                format!("{}", user.money),
                                false,
                            )
                        })
                    })
                    .await?;
            }
            Err(time) => {
                msg.channel_id
                    .send_message(&ctx.http, |builder| {
                        builder
                            .reference_message(msg)
                            .allowed_mentions(|f| f.replied_user(true));
                        builder.embed(|e| {
                            e.colour(0xff0069)
                                .title("Daily coins already claimed!")
                                .field(
                                    "You can claim your daily coins again in",
                                    format!("{}", time),
                                    false,
                                )
                        })
                    })
                    .await?;
            }
        }
    } else {
        msg.reply(ctx, "There was a problem getting the DB.")
            .await?;
    }
    Ok(())
}

#[command]
#[help_available]
#[num_args(2)]
#[only_in("guild")]
#[aliases("b")]
#[description("A command to get your daily cash.")]
async fn buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let item_id = args.single::<i32>()?;
    let item_amount = args.single::<i32>()?;
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let mut user: User = User::get(msg.author.id.0 as i64, db);
        let item: PurchasableItem = PurchasableItem::get_by_id(item_id, db).unwrap();
        if user.money >= item.price * item_amount {
            let inv_item: InventoryItem =
                InventoryItem::add(user.id, item_id, item_amount, db).unwrap();
            user.add_money(-(item.price * item_amount), db);
            msg.reply(
                ctx,
                format!(
                    "Succesfully purchased {} x{} for {}$ - You now have {}",
                    item.name,
                    item_amount,
                    item.price * item_amount,
                    inv_item.amount
                ),
            )
            .await?;
        } else {
            msg.reply(
                ctx,
                format!(
                    "You don't have enough money to purchase {} x{} for {}$",
                    item.name,
                    item_amount,
                    item.price * item_amount
                ),
            )
            .await?;
        }
    } else {
        msg.reply(ctx, "There was a problem getting the db.")
            .await?;
    }
    Ok(())
}

#[command]
#[help_available]
#[num_args(0)]
#[only_in("guild")]
#[aliases("$", "bal")]
#[description("Get your current balance")]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let user: User = User::get(msg.author.id.0 as i64, db);
        msg.reply(ctx, format!("You currently have {}$", user.money))
            .await?;
    } else {
        msg.reply(ctx, "There was a problem getting the db.")
            .await?;
    }
    Ok(())
}

#[command]
#[owners_only]
async fn new_item(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let item_name = args.single::<String>()?.replace("\"", "");
    let item_description = args.single::<String>()?.replace("\"", "");
    let item_price = args.single::<i32>()?;
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let item =
            PurchasableItem::create(item_name, item_description, item_price, &db);
        match item {
            Ok(item) => {
                msg.reply(
                    ctx,
                    format!("Item {} created with id {}.", item.name, item.id),
                )
                .await?;
            }
            Err(_) => {}
        }
    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
            .await?;
    }
    Ok(())
}

#[command]
#[owners_only]
async fn set_money(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let discord_user = args.single::<UserId>()?;
    let amount = args.single::<i32>()?;
    let data = ctx.data.read().await;

    if let Some(db) = data.get::<DatabaseContainer>() {
        let mut user: User = User::get(discord_user.0 as i64, db);
        user.set_money(amount, db);
        msg.reply(
            ctx,
            format!(
                "User {} ({}) now has {}$",
                UserId(user.discord_id as u64).mention(), user.id, user.money
            ),
        )
        .await?;
    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
            .await?;
    }
    Ok(())
}

#[command]
#[owners_only]
#[aliases("rd")]
async fn reset_daily(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        use crate::schema::users::dsl::*;
        diesel::update(users)
            .set(daily_claimed.eq(0))
            .execute(&db.get().unwrap())
            .expect("Unable to find user");

        msg.reply(ctx, format!("Daily reset.")).await?;
    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
            .await?;
    }
    Ok(())
}

#[command]
#[help_available]
#[num_args(0)]
#[only_in("guild")]
#[description("Get all purchasabale items.")]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let shop_items = PurchasableItem::get_all(db)?;
        msg.channel_id
            .send_message(&ctx.http, |builder| {
                builder
                    .reference_message(msg)
                    .allowed_mentions(|f| f.replied_user(true));
                builder.embed(|e| {
                    let embed = e.colour(0xff0069).title("Shop");
                    for item in shop_items {
                        embed.field(format!("[ID: {}] {} - ${}", item.id, item.name, item.price), format!("{}", item.description), false);
                    }
                    embed
                })
            })
            .await?;


    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
        .await?;
    }
    Ok(())
}

#[command]
#[help_available]
#[num_args(0)]
#[only_in("guild")]
#[aliases("inv")]
#[description("Get all items in your inventory.")]
async fn inventory(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let user = User::get(msg.author.id.0 as i64, db);
        let inv_items = user.get_inventory(db)?;
        msg.channel_id
            .send_message(&ctx.http, |builder| {
                builder
                    .reference_message(msg)
                    .allowed_mentions(|f| f.replied_user(true));
                builder.embed(|e| {
                    let embed = e.colour(0xff0069).title("Inventory");
                    for incomplete_item in inv_items {
                        let item = PurchasableItem::get_by_id(incomplete_item.item_id, db).unwrap();
                        embed.field(format!("[ID: {}] {} - You have: {}", item.id, item.name, incomplete_item.amount), format!("{}", item.description), false);
                    }
                    embed
                })
            })
            .await?;


    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
        .await?;
    }
    Ok(())
}