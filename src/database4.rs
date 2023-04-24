use std::collections::BTreeMap;
use std::fs;
use std::future::Future;
use actix_web::{HttpResponse, web};
use actix_web::http::StatusCode;
use handlebars::Handlebars;
use sqlx::postgres::PgPoolOptions;
use serde::Serialize;
use sqlx::{Error, Row};
use actix_web::HttpRequest;
use warp::{Rejection, Reply};
use warp::reject::Reject;

#[derive(sqlx::FromRow)]
#[derive(Serialize)]
pub struct User {
    title: String,
    description: String,
    name: String,
}

async fn England(_: HttpRequest) -> Result<(),sqlx::Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

   // let query = sqlx::query_as::<_, User>("SELECT title,description ,name FROM posts");
   // let users: Vec<User> = query.fetch_all(&pool).await?;


    let  rows = sqlx::query("SELECT title,description ,name FROM posts")
        .fetch_all(&pool)
        .await?;


    let mut handlebars = Handlebars::new();

    let index_template = fs::read_to_string("templates/index1.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();
    let mut data1:BTreeMap<String,String> = BTreeMap::new();
    let mut data5: BTreeMap<String, String> = BTreeMap::new();

    //  let mut data4:BTreeMpap<String,vec<users>> = BTree::new();
    let mut data2: BTreeMap<String, Box<[User]>> = BTreeMap::new();
    let mut data = BTreeMap::new();

    // for user in users{
    //     data1.insert("jof".parse().unwrap(), user.name);
    //     data1.insert("arc".parse().unwrap(), user.title);
    //     data1.insert("her".parse().unwrap(), user.description);
    // }
    for row in rows {
        let title:String=row.get("title");
        let description: String = row.get("description");
        let name:String= row.get("name");
        data.insert("one", title.clone());
        data.insert("two", description.clone());
        data.insert("three", name.clone());
    }


    let html = handlebars.render("index1", &data1).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);

    Ok(())

}