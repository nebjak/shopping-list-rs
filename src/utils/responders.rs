use actix_web::HttpResponse;
use mongodb::error::Result;
use serde::Serialize;

pub fn get_responder<T: Serialize>(input: Result<Option<T>>) -> HttpResponse
where
{
    match input {
        Ok(result) => match result {
            Some(items) => HttpResponse::Ok().json(items),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn post_responder<T: Serialize>(input: Result<T>) -> HttpResponse {
    match input {
        Ok(list_item) => HttpResponse::Ok().json(list_item),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
