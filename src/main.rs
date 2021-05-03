#[macro_use]
extern crate diesel;
// extern crate r2d2;
// extern crate r2d2_diesel;

mod commands;
mod database;
mod models;
mod schema;
mod utils;

use commands::admin::ADMIN_GROUP;
use commands::economy::ECONOMY_GROUP;
use commands::other::MY_HELP;
use commands::fun::FUN_GROUP;
use database::establish_connection;
use diesel::pg::PgConnection;
use diesel::r2d2;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        macros::{group, hook},
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
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
    type Value = DBPool;
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

#[group]
#[commands(quit, hello, db)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let db = establish_connection();
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
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&ECONOMY_GROUP)
        .group(&ADMIN_GROUP)
        .group(&FUN_GROUP);

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
