use futures::stream::StreamExt;
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, Bson},
    error::Result,
    Database,
};
use std::str::FromStr;

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
            .insert_one(bson::to_document(&shopping_list)?, None)
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
        let mut cursor = collection.find(doc! {}, None).await?;

        let mut result: Vec<ShoppingList> = Vec::new();

        while let Some(list) = cursor.next().await {
            result.push(bson::from_document(list?)?);
        }

        match result.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
        }
    }

    pub async fn get_one(&self, id: String) -> Result<Option<ShoppingList>> {
        let collection = self.db.collection(COLLECTION_NAME);

        let oid = ObjectId::from_str(id.as_str()).unwrap_or_default();

        match collection.find_one(doc! { "_id": oid }, None).await? {
            Some(document) => Ok(Some(bson::from_document(document).unwrap())),
            None => Ok(None),
        }
    }
}
