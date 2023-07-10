use actix_web::{post, web, HttpResponse, Responder, options};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct UnvalidatedPost {
    url: String,
    title: String
}

#[post("/addPost")]
pub async fn add_post(db: web::Data<SqlitePool>, post: web::Json<UnvalidatedPost>) -> impl Responder {
    let post = post.into_inner();

    println!("Received post: {} {}", post.url, post.title);

    let result = sqlx::query!(
        "INSERT INTO posts (url, title, created)
             VALUES (?1, ?2, unixepoch())",
        post.url,
        post.title
    )
    .execute(db.get_ref())
    .await;

    if let Err(error) = result {
        println!("Got error when inserting post: {}", error);
        HttpResponse::BadRequest().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
