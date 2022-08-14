use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web, App, HttpServer,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod api;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let frontend_origin =
        env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://127.0.0.1:5173".to_string());

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::resource("/tasks")
                    .route(web::get().to(api::index))
                    .route(web::post().to(api::create)),
            )
            .service(web::resource("/tasks/{task_id}").route(web::delete().to(api::destroy)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
