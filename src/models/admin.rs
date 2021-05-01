use crate::schema::bans;
use crate::schema::warnings;
use crate::DBPool;
use chrono::prelude::*;
use diesel::prelude::*;
use serenity::prelude::*;
use humantime::Duration;
use serenity::model::guild::{Guild, PartialGuild};
use serenity::model::id::{GuildId, UserId};


#[derive(Debug, Queryable)]
pub struct Ban {
    pub id: i32,
    pub banned_user_id: i32,
    pub admin_user_id: i32,
    pub guild_id: i64,
    pub reason: String,
    pub ban_time: i64,
    pub end_time: i64,
}

#[derive(Insertable)]
#[table_name = "bans"]
pub struct NewBan {
    pub banned_user_id: i32,
    pub admin_user_id: i32,
    pub guild_id: i64,
    pub reason: String,
    pub ban_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Queryable)]
pub struct Warning {
    pub id: i32,
    pub admin_user_id: i32,
    pub warned_user_id: i32,
    pub guild_id: i64,
    pub reason: String,
    pub warning_time: i64,
}

#[derive(Insertable)]
#[table_name = "warnings"]
pub struct NewWarning {
    pub admin_user_id: i32,
    pub warned_user_id: i32,
    pub guild_id: i64,
    pub reason: String,
    pub warning_time: i64,
}


impl NewBan {
    pub async fn commit(self, discord_uid: UserId, ctx: &Context, db: &DBPool) -> Result<Ban, String> {
        use crate::schema::bans::dsl::*;

        let guild: PartialGuild = Guild::get(ctx, GuildId(self.guild_id.clone() as u64)).await.unwrap();
        guild.ban_with_reason(ctx, discord_uid, 0, &self.reason).await.unwrap();
        let db = db.get().unwrap();
        match diesel::insert_into(bans)
            .values(self)
            .get_result::<Ban>(&db)
        {
            Ok(i) => Ok(i),
            Err(_) => Err("Couldn't create item.".to_string()),
        }
    }
}