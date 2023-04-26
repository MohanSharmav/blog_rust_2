
use std::collections::HashMap;
use std::{fmt, fs};
use std::any::Any;
use std::ptr::write;
use actix_web::{HttpResponse, Responder};
use sqlx::{FromRow, Row};
//use sqlx::postgres::PgRow;
use crate::model::connect_database::connect_database;
use futures::stream::BoxStream;
use sqlx::postgres::PgRow;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt::Formatter;
use crate::controller::home_page::get_all_posts_in_home_page;

#[derive(Deserialize, FromRow,Clone)]
pub struct posts{
pub title: String,
    pub description: String,
    pub name: String,
}
impl fmt::Display for posts{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{} {} {}",self.title,self.description,self.name)
    }
}

 impl fmt::Debug for posts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi")
    }
}
pub async fn get_all_posts()->Result<Vec<posts>,Box<dyn Error>>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

  let q="select title,description,name from posts";
  let query=sqlx::query_as::<_, posts>(q);

    let rows= query.fetch_all(&pool).await?;

    // let posting= rows.iter().map(|row| {
    //     pots {
    //         title: row.get("title"),
    //         description: row.get("description"),
    //         name: row.get("name"),
    //     }
    // } ).collect();

    get_all_posts_in_home_page(rows.clone());
   println!("ASD");
    Ok(rows.clone())
}


