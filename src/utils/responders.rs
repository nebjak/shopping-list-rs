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
