mod database;
mod database2;
mod database3;
mod database4;

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
use database::selecting;
use database2::finally;

use warp::{Rejection, Reply};
use crate::database::{get_data, will_win};
use crate::database3::England;

async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("src/one.html");
     Ok(NamedFile::open(path)?)
}






#[tokio::main]
async fn main() -> Result<()>{

finally().await.expect("ASDASd");


     let mut handlebars = Handlebars::new();
     handlebars.register_template_file("index", "./templates/index.hbs")
         .unwrap();

     //database
selecting().await.expect("TODO: panic message");


     HttpServer::new(|| {
          App::new()
            //  .server(web::resource("/ben").to(England))
             // .service(web::resource("/roy").to(finally()))
              .service(web::resource("/hi").to(index))
            //  .service(web::resource("/data").to(get_data))
              .service(web::resource("/benstokes").to(will_win))
              .service(web::resource("/king").to(England()))
          //   .service(web::resource("/we").to(finally()))
             // .service(web::resource("/").to(finally()))
              // .service(web::resource("/we").to(|| async {
              //      finally().await.expect("TODO: panic message");
              //      Ok::<HttpResponse, sqlx::Error>(HttpResponse::Ok().finish())
              // }))

     })


         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
