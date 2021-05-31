use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use mongodb::{bson::oid::ObjectId, Client, Database};

use crate::models::shopping_list::{ListItem, ShoppingList};
use crate::services::list_item_service::ListItemService;
use crate::services::shopping_list_service::ShoppingListService;
use crate::utils::responders::{get_responder, post_responder, put_responder};
use std::str::FromStr;

mod models;
mod services;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("")]
async fn add_shopping_list(
    db_client: web::Data<DbConnection>,
    shopping_list: web::Json<ShoppingList>,
) -> impl Responder {
    let shopping_list_service = ShoppingListService::new(&db_client.db);
    post_responder(shopping_list_service.add(shopping_list.into_inner()).await)
}

#[get("")]
async fn get_shopping_lists(db_client: web::Data<DbConnection>) -> impl Responder {
    let shoppling_list_service = ShoppingListService::new(&db_client.db);

    get_responder(shoppling_list_service.get_all().await)
}

#[get("/{shopping_list_id}")]
async fn get_one_shopping_list(
    db_client: web::Data<DbConnection>,
    shopping_list_id: web::Path<String>,
) -> impl Responder {
    let shopping_list_service = ShoppingListService::new(&db_client.db);

    get_responder(
        shopping_list_service
            .get_one(shopping_list_id.into_inner())
            .await,
    )
}

#[post("/{shopping_list_id}/items")]
async fn add_list_item(
    db_client: web::Data<DbConnection>,
    shopping_list_id: web::Path<String>,
    list_item: web::Json<ListItem>,
) -> impl Responder {
    let list_item_service = ListItemService::new(&db_client.db);

    let shopping_list_id = match ObjectId::from_str(shopping_list_id.into_inner().as_str()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    post_responder(
        list_item_service
            .add(shopping_list_id, list_item.into_inner())
            .await,
    )
}

#[get("/{shopping_list_id}/items")]
async fn get_list_items(
    db_client: web::Data<DbConnection>,
    shopping_list_id: web::Path<String>,
) -> impl Responder {
    let shopping_list_id = match ObjectId::from_str(shopping_list_id.into_inner().as_str()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let list_item_service = ListItemService::new(&db_client.db);

    get_responder(list_item_service.get_all(shopping_list_id).await)
}

#[put("/{shopping_list_id}/items/{list_item_id}")]
async fn update_list_item(
    db_client: web::Data<DbConnection>,
    path: web::Path<(String, String)>,
    list_item: web::Json<ListItem>,
) -> impl Responder {
    let path = path.into_inner();
    let list_item_id = match ObjectId::from_str(path.1.as_str()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let list_item_service = ListItemService::new(&db_client.db);

    put_responder(
        list_item_service
            .update(list_item_id, list_item.into_inner())
            .await,
    )
}

struct DbConnection {
    db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    const SERVER_URL: &str = dotenv!("SERVER_URL");
    const SERVER_PORT: &str = dotenv!("SERVER_PORT");
    const DATABASE_URL: &str = dotenv!("DATABASE_URL");
    const DATABASE_NAME: &str = dotenv!("DATABASE_NAME");

    let client = Client::with_uri_str(DATABASE_URL)
        .await
        .expect("⛔️ DB connection filed!");

    println!(
        "👌 Server is running on http://{}:{}/api/v1/",
        SERVER_URL, SERVER_PORT
    );

    HttpServer::new(move || {
        App::new()
            .data(DbConnection {
                db: client.database(DATABASE_NAME),
            })
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/shopping-lists")
                            .service(update_list_item)
                            .service(add_list_item)
                            .service(get_list_items)
                            .service(add_shopping_list)
                            .service(get_one_shopping_list)
                            .service(get_shopping_lists),
                    )
                    .service(hello),
            )
    })
    .bind(format!("{}:{}", SERVER_URL, SERVER_PORT))?
    .run()
    .await
}
