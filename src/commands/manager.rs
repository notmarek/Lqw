use crate::{DatabaseContainer, ShardManagerContainer};
use serenity::framework::standard::{macros::command, CommandResult};
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
    msg.reply(ctx, format!("hello {}", msg.author.id.mention()))
        .await?;
    Ok(())
}

#[command]
#[owners_only]
async fn db(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<DatabaseContainer>() {
        let state = db.state();

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

