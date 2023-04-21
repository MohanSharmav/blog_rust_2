mod database;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web};
use actix_web::http::StatusCode;
use handlebars::Handlebars;
use tokio::select;
use warp::reply::with_status;
use database::selecting;


use warp::{Rejection, Reply};
use crate::database::get_data;

async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("src/one.html");
     Ok(NamedFile::open(path)?)
}

async fn will_win()->HttpResponse{
     let mut handlebars = Handlebars::new();

     // Register the "index" template from a file
     let index_template = fs::read_to_string("templates/index.hbs").unwrap();
     handlebars
         .register_template_string("index", &index_template)
         .unwrap();

     // Create a context with data
     let mut data = BTreeMap::new();
     data.insert("title", "My Page Title BEN STOKES");
     data.insert("header", "Welcome to my page");

     // Render the template with the context
     let html = handlebars.render("index", &data).unwrap();

     // Return the HTML page as a response
     HttpResponse::Ok()
         .content_type("text/html; charset=utf-8")
         .body(html)
}

#[tokio::main]
async fn main() -> Result<()>{
     let mut handlebars = Handlebars::new();
     handlebars.register_template_file("index", "./templates/index.hbs")
         .unwrap();

     //database
selecting().await.expect("TODO: panic message");


     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(index))
              .service(web::resource("/hi").to(index))
              .service(web::resource("/data").to(get_data))
              .service(web::resource("/benstokes").to(will_win))

     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
