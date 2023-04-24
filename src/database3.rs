use std::collections::BTreeMap;
use std::fs;
use actix_web::HttpResponse;
use handlebars::Handlebars;
use sqlx::postgres::PgPoolOptions;
use serde::Serialize;
use sqlx::Error;

#[derive(sqlx::FromRow)]
#[derive(Serialize)]
pub struct User {
    title: String,
    description: String,
    name: String,
}

pub async fn England()->Result<(),Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let query = sqlx::query_as::<_, User>("SELECT title,description ,name FROM posts");
    let users: Vec<User> = query.fetch_all(&pool).await?;


    let mut handlebars = Handlebars::new();

    let index_template = fs::read_to_string("templates/index1.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();
let mut data1:BTreeMap<String,String> = BTreeMap::new();
    let mut data5: BTreeMap<String, String> = BTreeMap::new();

   //  let mut data4:BTreeMpap<String,vec<users>> = BTree::new();
    let mut data2: BTreeMap<String, Box<[User]>> = BTreeMap::new();

    for user in users{
        data1.insert("jof".parse().unwrap(), user.name);
        data1.insert("arc".parse().unwrap(), user.title);
        data1.insert("her".parse().unwrap(), user.description);
    }

    let html = handlebars.render("index1", &data1).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);

Ok(())

}