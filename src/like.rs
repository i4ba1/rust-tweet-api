use actix_web::web::Path;
use actix_web::HttpResponse;
use crate::constants::APPLICATION_JSON;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like{
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    let likes = Likes{results: vec![]};
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(likes)
}

#[post("/plusOne/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>) -> HttpResponse{
    let like = Like::new();
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/minusOne/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>) -> HttpResponse{
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}