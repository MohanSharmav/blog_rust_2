
mod controller;
mod model;
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
//use crate::controller::home_page::default_page;
use warp::{Rejection, Reply};
use crate::controller::home_page::default_page;
use crate::model::Homepage_query::get_all_posts;

//use crate::model::Homepage_query::get_all_posts;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
//Test start

    let x=  get_all_posts().await;
    println!("wewe ");
    println!("we believe {:?}", x);
    println!("ASDSADADASDasdadsdasd");
     //Test End
     HttpServer::new(|| {
          App::new()
             .service(web::resource("/").route(web::get().to(default_page)))
              // .service(web::resource("/").route(web::get().to(get_all_posts)))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");



     Ok(())
}
