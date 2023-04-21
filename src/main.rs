mod database;

use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use database::selecting;


use warp::{Rejection, Reply};


async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("src/one.html");
     Ok(NamedFile::open(path)?)
}


#[tokio::main]
async fn main() -> Result<()>{


     //database
selecting().await.expect("TODO: panic message");


     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(index))
              .service(web::resource("/hi").to(index))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
