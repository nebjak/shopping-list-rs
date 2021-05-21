use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum ItemStatus {
    TODO,
    DONE,
    REMOVED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShoppingList {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub status: Option<ItemStatus>,
    pub shopping_list_id: Option<ObjectId>,
}
