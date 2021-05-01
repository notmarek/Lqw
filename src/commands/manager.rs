use crate::models::economy::PurchasableItem;
use crate::models::user::User;
use crate::{DatabaseContainer, ShardManagerContainer};
use diesel::prelude::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;
    }
    Ok(())
}

#[command]
#[owners_only]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    // let data = ctx.data.read().await;
    msg.reply(ctx, format!("hello <@{}>", msg.author.id))
        .await?;
    Ok(())
}

#[command]
#[owners_only]
async fn db(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let db = &*db.lock().await;
        let state: r2d2::State = db.state();

        msg.reply(
            ctx,
            format!(
                "Connections: {} Idle: {}",
                state.connections, state.idle_connections
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
            .execute(&*db.lock().await.get().unwrap())
            .expect("Unable to find user");

        msg.reply(ctx, format!("Daily reset.")).await?;
    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
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
            PurchasableItem::create(item_name, item_description, item_price, &*db.lock().await);
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
        let db = &*db.lock().await;
        let mut user: User = User::get(discord_user.0 as i64, db);
        user.set_money(amount, db);
        msg.reply(ctx, format!("User <@{}> ({}) now has {}$", user.discord_id, user.id, user.money)).await?;
    } else {
        msg.reply(ctx, "There's been a problem getting the DB.")
        .await?;
    }
    Ok(())
}
