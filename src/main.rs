mod database;

use std::path::Path;
use actix::ContextFutureSpawner;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool, query_as_with};
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;

use warp::{Rejection, Reply};


async fn index(req: HttpRequest)-> Result<NamedFile>{
    // let x="Boss";


     let path= Path::new("src/one.html");
     Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<()>{


     dotenv::dotenv().expect("Unable to load environment variables from .env file");

     let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

     let mut pool = PgPoolOptions::new()
         .max_connections(100)
         .connect(&db_url)
         .await.expect("Unable to connect to Postgres");

// (&pool).await?;




     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(index))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
