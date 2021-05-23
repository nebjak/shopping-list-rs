use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Serialize)]
pub enum ItemStatus {
    TODO,
    DONE,
    REMOVED,
}

impl fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ItemStatus::TODO => write!(f, "TODO"),
            ItemStatus::DONE => write!(f, "DONE"),
            ItemStatus::REMOVED => write!(f, "REMOVED"),
        }
    }
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
