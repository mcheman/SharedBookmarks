use actix_web::{get, web, HttpResponse, Responder};
use chrono::{TimeZone, Local};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Serialize)]
struct Post {
    url: String,
    title: String,
    favicon_url: Option<String>,
    created_formatted: String,
}

#[get("/")]
async fn index(db: web::Data<SqlitePool>, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let posts: Vec<Post> =
        sqlx::query!("SELECT url, title, favicon_url, created FROM posts ORDER BY created DESC")
            .fetch_all(db.get_ref())
            .await
            .unwrap_or(Vec::new())
            .iter()
            .map(|r| Post {
                url: r.url.clone(),
                title: r.title.clone(),
                favicon_url: r.favicon_url.clone(),
                created_formatted: Local.timestamp_opt(r.created, 0).unwrap().format("%A %B %e, %Y at %I:%M %P").to_string(),
            })
            .collect();

    let body = hb
        .render("index", &json!({ "posts": posts }))
        .expect("index.hbs should exist in ./templates and contain valid handlebars syntax");
    HttpResponse::Ok().body(body)
}
