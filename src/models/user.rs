use crate::schema::users;
use crate::DBPool;
use chrono::prelude::*;
use diesel::prelude::*;
use humantime::Duration;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: i64,
    pub messages: i32,
    pub money: i32,
    pub daily_claimed: i64,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub discord_id: i64,
}

impl User {
    pub fn get(discord_user_id: i64, db: &DBPool) -> Self {
        use crate::schema::users::dsl::*;
        let db = db.get().unwrap();
        match users
            .filter(discord_id.eq(&discord_user_id))
            .first::<User>(&db)
        {
            Ok(user) => user,
            Err(_) => {
                match diesel::insert_into(users)
                    .values(discord_id.eq(discord_user_id))
                    .get_result::<User>(&db)
                {
                    Ok(u) => u,
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }

    pub fn claim_daily(&mut self, pool: &DBPool) -> Result<bool, String> {
        use crate::schema::users::dsl::*;
        let now: DateTime<Utc> = Utc::now();
        let db = pool.get().unwrap();
        if self.daily_claimed + 86400 <= now.timestamp() {
            // Make sure more than a day has passed since last claim
            self.add_money(20, &pool);
            diesel::update(users.find(self.id))
                .set(daily_claimed.eq(now.timestamp()))
                .execute(&db)
                .expect("Unable to find user");
            self.daily_claimed = now.timestamp();
            Ok(true)
        } else {
            let next_claim = self.daily_claimed + 86400 - now.timestamp();
            let next_claim = Duration::from(std::time::Duration::new(next_claim as u64, 0));
            Err(next_claim.to_string())
        }
    }

    pub fn add_money(&mut self, amount: i32, db: &DBPool) {
        self.set_money(self.money + amount, db)
    }

    pub fn set_money(&mut self, amount: i32, db: &DBPool) {
        use crate::schema::users::dsl::*;
        let db = db.get().unwrap();
        diesel::update(users.find(self.id))
            .set(money.eq(amount))
            .execute(&db)
            .expect("Unable to find user");
        self.money = amount;
    }
}
