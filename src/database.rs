use std::collections::BTreeMap;
use std::fs;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use warp::http::Response;
use handlebars::Handlebars;
use std::string::String;
use warp::reply::html;


pub async fn select_all_from_table() -> Result<(),Error> {

    let mut handlebars = Handlebars::new();

    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

    let mut data = BTreeMap::new();

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let rows = sqlx::query("SELECT title,description,name FROM posts")
          .fetch_all(&pool)
          .await?;
    for row in rows{
        let title:String=row.get("title");
       let description: String = row.get("description");
        let name:String= row.get("name");
        data.insert("title", title.clone());
        data.insert("description", description.clone());
        data.insert("name", name.clone());

    }
    let html = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
    Ok(())
}

pub async fn controller(x:String)->HttpResponse{
    let mut handlebars = Handlebars::new();

    // Register the "index" template from a file
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

// Testing - START

// Testing - END

    // Create a context with data
    let mut data = BTreeMap::new();

    // Render the template with the context
    let html = handlebars.render("index", &data).unwrap();

    // Return the HTML page as a response
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

