#[macro_use]
extern crate diesel;
// extern crate r2d2;
// extern crate r2d2_diesel;

mod commands;
mod database;
mod models;
mod schema;

use commands::other::MY_HELP;
use commands::economy::ECONOMY_GROUP;
use commands::admin::ADMIN_GROUP;
use database::establish_connection;
use diesel::pg::PgConnection;
use diesel::r2d2;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        event::ResumedEvent,
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

use std::{collections::HashSet, env, sync::Arc};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::manager::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
pub type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Arc<Mutex<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        let activity = serenity::model::gateway::Activity::competing("Your Mom");
        // let status = serenity::model::user::OnlineStatus::Online;
        ctx.set_activity(activity).await;
    }   

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    msg.reply(ctx, format!("Could not find command named '{}'", unknown_command_name)).await.unwrap();
}


#[group]
#[commands(quit, hello, db)]
struct General;


#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let db = establish_connection();
    let db = Arc::new(Mutex::new(db));
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .unrecognised_command(unknown_command)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&ECONOMY_GROUP)
        .group(&ADMIN_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<DatabaseContainer>(db.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
