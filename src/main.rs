mod database;


use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, Result, web};
use actix_web::http::StatusCode;
use handlebars::Handlebars;
use sqlx::pool;
use tokio::select;
use warp::reply::{html, with_status};

use warp::{Rejection, Reply};
use crate::database::{controller};

async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("src/one.html");
     Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<()>{

     HttpServer::new(|| {
          App::new()
              .service(web::resource("/benstokes").route(web::get().to(controller)))

     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
