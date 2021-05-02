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
#[commands(ban, warn)]
struct Admin;

#[command]
#[required_permissions("BAN_MEMBERS")]
#[help_available]
#[only_in("guild")]
#[aliases("yeet")]
#[description("A command to ban people.")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let discord_uid = args.single::<UserId>()?;
    let duration = args.single::<String>()?;
    let reason = args.rest();
    if let Some(db) = data.get::<DatabaseContainer>() {
        let admin = User::get(msg.author.id.0 as i64, db);
        let user = User::get(discord_uid.0 as i64, db);
        let ban = user.ban(ctx, msg.guild_id.unwrap().0 as i64, admin, reason.to_string(), 0, db).await?;
        msg.reply(ctx, format!("User banned successfully~, ban_id: {}", ban.id)).await?;
    } else {
        msg.reply(ctx, "Error getting DB.").await?;
    }
    Ok(())
}

#[command]
#[required_permissions("KICK_MEMBERS")]
#[help_available]
#[only_in("guild")]
#[description("A command to warn people.")]
async fn warn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let discord_uid = args.single::<UserId>()?;
    let reason = args.rest();
    if let Some(db) = data.get::<DatabaseContainer>() {
        let admin = User::get(msg.author.id.0 as i64, db);
        let warnee = User::get(discord_uid.0 as i64, db);
        let warn = warnee.warn(admin, msg.guild_id.unwrap().0 as i64, reason.to_string(), db)?;
        msg.reply(ctx, format!("{} You have been warned for: `{}`, warn_id: {}", discord_uid.mention(), reason, warn.id)).await?;
    } else {
        msg.reply(ctx, "Error getting DB.").await?;
    }
    Ok(())
}

