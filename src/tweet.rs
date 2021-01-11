use uuid::Uuid;
use crate::like::Like;
use actix_web::HttpResponse;
use actix_web::web::Json;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;
use crate::response::Response;
use crate::{DBPool, DBPooledConnection};
use actix_web::web::Path;

use super::schema::tweets;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use std::str::FromStr;

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet{
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>
}

impl Tweet{
    pub fn new(message:String) -> Self{
        Self{
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest{
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list() -> HttpResponse{
    let tweets = Tweets{results: vec![]};
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[post("/newTweet")]
pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse{
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(tweet_req.to_tweet())
}

#[get("/view_detail_tweet/{id}")]
pub async fn view_detail_tweet(path: Path<(String,)>) -> HttpResponse{
    let found_tweet: Option<Tweet> = None;

    match found_tweet {
        Some(tweet) =>HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/deleteTweet/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse{
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
