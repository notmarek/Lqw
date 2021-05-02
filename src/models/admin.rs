use crate::models::user::User;
use crate::schema::bans;
use crate::schema::warnings;
use crate::DBPool;
use chrono::prelude::*;
use diesel::prelude::*;
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::*;

#[derive(Debug, Queryable)]
pub struct Ban {
    pub id: i32,
    pub banned_user_id: i32,
    pub admin_user_id: i32,
    pub guild_id: i64,
    pub reason: String,
    pub ban_time: i64,
    pub end_time: i64,
    pub lifted: i64,
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
    pub async fn commit(
        self,
        discord_uid: UserId,
        ctx: &Context,
        db: &DBPool,
    ) -> Result<Ban, String> {
        use crate::schema::bans::dsl::*;
        GuildId(self.guild_id.clone() as u64)
            .ban_with_reason(ctx, discord_uid, 0, &self.reason)
            .await
            .unwrap();
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

impl Ban {
    pub fn get(user: &User, discord_guild_id: i64, db: &DBPool) -> Result<Self, String> {
        use crate::schema::bans::dsl::*;
        let db = db.get().unwrap();
        match bans
            .filter(banned_user_id.eq(&user.id))
            .filter(guild_id.eq(discord_guild_id))
            .filter(lifted.eq(0))
            .first::<Ban>(&db)
        {
            Ok(ban) => Ok(ban),
            Err(_) => Err("Ban not found".to_string()),
        }
    }
    pub async fn lift(&mut self, discord_id: UserId, ctx: &Context, db: &DBPool) {
        use crate::schema::bans::dsl::*;
        let db = db.get().unwrap();
        let now: DateTime<Utc> = Utc::now();
        GuildId(self.guild_id.clone() as u64)
            .unban(ctx, discord_id)
            .await
            .unwrap();
        diesel::update(bans.find(self.id))
            .set(lifted.eq(now.timestamp()))
            .execute(&db)
            .expect("Unable to find ban");
    }
}

impl Warning {
    pub fn new(
        admin: User,
        warnee: User,
        i_guild_id: i64,
        i_reason: String,
        db: &DBPool,
    ) -> Result<Self, String> {
        use crate::schema::warnings::dsl::*;
        let db = db.get().unwrap();
        let now: DateTime<Utc> = Utc::now();
        let new_warn = NewWarning {
            admin_user_id: admin.id,
            warned_user_id: warnee.id,
            guild_id: i_guild_id,
            reason: i_reason,
            warning_time: now.timestamp(),
        };
        match diesel::insert_into(warnings)
            .values(new_warn)
            .get_result::<Warning>(&db)
        {
            Ok(i) => Ok(i),
            Err(_) => Err("Couldn't create warning.".to_string()),
        }
    }
}
