use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson};
use mongodb::error::Result;
use mongodb::{bson, Collection, Database};

use crate::models::shopping_list::{ItemStatus, ListItem};
use futures::StreamExt;
use mongodb::results::{DeleteResult, UpdateResult};

pub struct ListItemService<'a> {
    db: &'a Database,
    collection: Collection,
}

impl ListItemService<'_> {
    pub fn new(db: &Database) -> ListItemService {
        ListItemService {
            db,
            collection: db.collection("listItems"),
        }
    }

    pub async fn add(
        &self,
        shopping_list_id: ObjectId,
        mut list_item: ListItem,
    ) -> Result<ListItem> {
        list_item.shopping_list_id = Some(shopping_list_id);

        list_item.status = Some(ItemStatus::TODO);
        match self
            .collection
            .insert_one(bson::to_document(&list_item)?, None)
            .await
        {
            Ok(result) => {
                list_item.id = match result.inserted_id {
                    Bson::ObjectId(id) => Some(id),
                    _ => None,
                };
                Ok(list_item)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(err)
            }
        }
    }

    pub async fn get_all(&self, shopping_list_id: ObjectId) -> Result<Option<Vec<ListItem>>> {
        let mut cursor = self
            .collection
            .find(
                doc! {
                    "shopping_list_id": shopping_list_id
                },
                None,
            )
            .await?;

        let mut result: Vec<ListItem> = Vec::new();

        while let Some(list_item) = cursor.next().await {
            result.push(bson::from_document(list_item?)?);
        }

        match result.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
        }
    }

    pub async fn update(
        &self,
        list_item_id: ObjectId,
        list_item: ListItem,
    ) -> Result<UpdateResult> {
        self.collection
            .update_one(
                doc! {"_id": list_item_id},
                doc! {
                    "$set": {
                        "name": list_item.name,
                        "status": list_item.status.unwrap().to_string()
                    }
                },
                None,
            )
            .await
    }

    pub async fn delete(&self, list_item_id: ObjectId) -> Result<DeleteResult> {
        self.collection
            .delete_one(doc! {"_id": list_item_id}, None)
            .await
    }
}
