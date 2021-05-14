use mongodb::{bson, bson::Bson, error::Result, Database};

use crate::models::shopping_list::ShoppingList;

pub struct ShoppingListService<'a> {
    db: &'a Database,
}

impl ShoppingListService<'_> {
    pub fn new(db: &Database) -> ShoppingListService {
        ShoppingListService { db }
    }

    pub async fn add(&self, mut shopping_list: ShoppingList) -> Result<ShoppingList> {
        let collection = self.db.collection("shoppingLists");
        match collection
            .insert_one(bson::to_document(&shopping_list).unwrap(), None)
            .await
        {
            Ok(result) => {
                shopping_list.id = match result.inserted_id {
                    Bson::ObjectId(id) => Some(id),
                    _ => None,
                };
                Ok(shopping_list)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(err)
            }
        }
    }
}
