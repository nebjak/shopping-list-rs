use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::error::Result;
use mongodb::{bson, Collection, Database};

use crate::models::shopping_list::{ItemStatus, ListItem};

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
}
