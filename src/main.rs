use std::env;
use dotenv::dotenv;
use actix_web::middleware::Logger;
use actix_web::{ get, post, web::Json, App, HttpResponse, HttpServer, Responder };
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

mod models;

use models::{ ResponseUser, RequestUser };

#[get("/user")]
async fn all_users_handler() -> impl Responder {
    let pool = establish_connection().await;
    let records = sqlx::query_as!(ResponseUser, "SELECT * FROM users;")
        .fetch_all(&pool)
        .await
        .expect("Error retrieving records");

    /*
    let mut response = Vec::new();

    for record in records {
        response.push(ResponseUser {
            id: record.id,
            name: record.name,
            age: record.age
        });
    }
    let response = json!(response);
    */

    let response = json!(records);

    pool.close().await;

    HttpResponse::Ok().json(json!({ "status": "ok", "response": response }))
}

#[post("/user/new")]
async fn new_user_handler(data: Json<RequestUser>) -> impl Responder {
    let data = data.into_inner();
    let pool = establish_connection().await;
    let user_id = Uuid::new_v4().hyphenated().to_string();

    let res = sqlx::query!(
        "INSERT INTO users VALUES ($1, $2, $3) RETURNING id;",
        user_id, data.name, data.age)
        .fetch_one(&pool)
        .await
        .expect("Error inserting user");

    let response = json!({
        "status": "ok",
        "id": res.id,
    });

    pool.close().await;

    HttpResponse::Ok().json(response)
}

async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let db_url = &env::var("DATABASE_URL")
        .expect("Error finding db connection");

    sqlx::PgPool::connect(db_url)
        .await
        .expect("Error connecting to db")
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(new_user_handler)
            .service(all_users_handler)
            .wrap(Logger::default())
    })
    .bind("192.168.102.215:4000")?
    .run()
    .await?;

    Ok(())
}
