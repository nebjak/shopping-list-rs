use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use mongodb::{Client, Database};

use crate::models::shopping_list::ShoppingList;
use crate::services::shopping_list_service::ShoppingListService;

mod models;
mod services;

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
    match shopping_list_service.add(shopping_list.into_inner()).await {
        Ok(shopping_list) => HttpResponse::Ok().json(shopping_list),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("")]
async fn get_shopping_lists(db_client: web::Data<DbConnection>) -> impl Responder {
    let shoppling_list_service = ShoppingListService::new(&db_client.db);

    match shoppling_list_service.get_all().await {
        Ok(result) => match result {
            Some(lists) => HttpResponse::Ok().json(lists),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/{shopping_list_id}")]
async fn get_one_shopping_list(
    db_client: web::Data<DbConnection>,
    shopping_list_id: web::Path<String>,
) -> impl Responder {
    let shopping_list_service = ShoppingListService::new(&db_client.db);

    match shopping_list_service
        .get_one(shopping_list_id.into_inner())
        .await
    {
        Ok(result) => match result {
            Some(list) => HttpResponse::Ok().json(list),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/tmpdemo")]
async fn tmpdemo() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

struct DbConnection {
    db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    const DATABASE_URL: &str = dotenv!("DATABASE_URL");
    const DATABASE_NAME: &str = dotenv!("DATABASE_NAME");

    let client = Client::with_uri_str(DATABASE_URL).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .data(DbConnection {
                db: client.database(DATABASE_NAME),
            })
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/shopping-lists")
                            .service(add_shopping_list)
                            .service(get_one_shopping_list)
                            .service(get_shopping_lists),
                    )
                    .service(hello)
                    .service(tmpdemo),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}