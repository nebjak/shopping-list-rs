use futures::stream::StreamExt;
use mongodb::{
    bson,
    bson::{doc, Bson},
    error::Result,
    Database,
};

use crate::models::shopping_list::ShoppingList;

const COLLECTION_NAME: &'static str = "shoppingLists";

pub struct ShoppingListService<'a> {
    db: &'a Database,
}

impl ShoppingListService<'_> {
    pub fn new(db: &Database) -> ShoppingListService {
        ShoppingListService { db }
    }

    pub async fn add(&self, mut shopping_list: ShoppingList) -> Result<ShoppingList> {
        let collection = self.db.collection(COLLECTION_NAME);
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

    pub async fn get_all(&self) -> Result<Option<Vec<ShoppingList>>> {
        let collection = self.db.collection(COLLECTION_NAME);
        let mut cursor = collection.find(doc! {}, None).await.unwrap();

        let mut result: Vec<ShoppingList> = Vec::new();

        while let Some(list) = cursor.next().await {
            result.push(bson::from_document(list.unwrap()).unwrap());
        }

        match result.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
        }
    }
}
