use crate::schema::inventory;
use crate::schema::shop;
use crate::models::user::User;
use crate::DBPool;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct PurchasableItem {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub purchasable: bool,
}
#[derive(Insertable)]
#[table_name = "shop"]
pub struct NewPurchasableItem {
    pub name: String,
    pub description: String,
    pub price: i32,
}

impl PurchasableItem {
    pub fn get_by_id(item_id: i32, db: &DBPool) -> Result<Self, String> {
        use crate::schema::shop::dsl::*;
        let db = db.get().unwrap();
        match shop.filter(id.eq(item_id)).first::<PurchasableItem>(&db) {
            Ok(item) => Ok(item),
            Err(_) => Err("Item not found.".to_string()),
        }
    }
    pub fn get_all(db: &DBPool) -> Result<Vec<Self>, String> {
        use crate::schema::shop::dsl::*;
        let db = db.get().unwrap();
        match shop.filter(purchasable.eq(true)).get_results::<PurchasableItem>(&db) {
            Ok(items) => Ok(items),
            Err(_) => Err("Couldn't fetch all items.".to_string()),
        }
    }
    pub fn create(
        item_name: String,
        item_description: String,
        item_price: i32,
        db: &DBPool,
    ) -> Result<Self, String> {
        use crate::schema::shop::dsl::*;
        let item = NewPurchasableItem {
            name: item_name,
            description: item_description,
            price: item_price,
        };
        let db = db.get().unwrap();
        match diesel::insert_into(shop)
            .values(item)
            .get_result::<PurchasableItem>(&db)
        {
            Ok(i) => Ok(i),
            Err(_) => Err("Couldn't create item.".to_string()),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct InventoryItem {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "inventory"]
pub struct NewInventoryItem {
    pub user_id: i32,
    pub item_id: i32,
    pub amount: i32,
}

impl InventoryItem {
    pub fn get_by_id(inv_user_id: i32, inv_item_id: i32, db: &DBPool) -> Result<Self, String> {
        use crate::schema::inventory::dsl::*;
        let db = db.get().unwrap();
        match inventory
            .filter(user_id.eq(inv_user_id))
            .filter(item_id.eq(inv_item_id))
            .first::<InventoryItem>(&db)
        {
            Ok(item) => Ok(item),
            Err(_) => Err("Item not found.".to_string()),
        }
    }
    pub fn get_all_by_user(user: User, db: &DBPool) -> Result<Vec<Self>, String> {
        use crate::schema::inventory::dsl::*;
        let db = db.get().unwrap();
        match inventory.filter(user_id.eq(user.id)).get_results::<Self>(&db) {
            Ok(items) => Ok(items),
            Err(_) => Err("Couldn't fetch all items.".to_string()),
        }
    }
    pub fn add(
        inv_user_id: i32,
        inv_item_id: i32,
        inv_amount: i32,
        db: &DBPool,
    ) -> Result<Self, String> {
        use crate::schema::inventory::dsl::*;
        let db = db.get().unwrap();
        let mut item: Self;
        match inventory
            .filter(user_id.eq(inv_user_id))
            .filter(item_id.eq(inv_item_id))
            .first::<InventoryItem>(&db)
        {
            Ok(i) => {
                diesel::update(inventory.find(i.id))
                    .set(amount.eq(i.amount + inv_amount))
                    .execute(&db)
                    .expect("Unable to find inventory item");
                item = i;
                item.amount += inv_amount;
            }
            Err(_) => {
                let new_item = NewInventoryItem {
                    user_id: inv_user_id,
                    item_id: inv_item_id,
                    amount: inv_amount,
                };
                match diesel::insert_into(inventory)
                    .values(new_item)
                    .get_result::<InventoryItem>(&db)
                {
                    Ok(i) => item = i,
                    Err(_) => return Err("Couldn't create item.".to_string()),
                }
            }
        }
        Ok(item)
    }
}
