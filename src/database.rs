use std::collections::BTreeMap;
use std::fs;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use warp::http::Response;
use handlebars::Handlebars;
use std::string::String;
use warp::reply::html;
//
// pub  async fn connect_database(){
//
//     dotenv::dotenv().expect("Unable to load environment variables from .env file");
//
//     let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
//
//     let  mut pool = PgPoolOptions::new()
//         .max_connections(100)
//         .connect(&db_url)
//         .await.expect("Unable to connect to Postgres");
//
// }
//// let rows = sqlx::query("SELECT name FROM categories")
//     //     .map(|row:sqlx::postgres::PgRow|{
//     //          let a:String=row.get("name");
//     //       println!("{:?}",rows)
//     //     })
//     //     .fetch_all(pool)
//     //     .await?;

pub(crate) async fn selecting() -> Result<(),Error>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    select_all_from_table().await.expect("cant select");

    println!("⭐⭐⭐⭐⭐⭐");
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await?;

    for row in rows{
        let name: String=row.get("name");
        println!("name is ⭐: {}",name)
    }
    Ok(())
}


pub async fn select_all_from_table() -> Result<(),Error> {

    let mut handlebars = Handlebars::new();

    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

    let mut data = BTreeMap::new();
    //  data.insert("title", benny);
    data.insert("ji", "Jos buttler".parse().unwrap());


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
        data.insert("one", title.clone());
        data.insert("two", description.clone());
        data.insert("three", name.clone());
      //  let x:String=title+ &*description + &*name;
        println!("⭐⭐⭐⭐⭐⭐wiejwidni{}", title.clone());
        println!("{}", description.clone());
        println!("{}", name.clone());
     //   get_data(title, description, name).await;
        let x: String = title.to_owned() + &*description + &*name;
        will_win(x.clone()).await;
        println!("boissojsdsodojsdo{}",x);
    }
    let html = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
    Ok(())
}

pub async fn get_data(title:String,description:String,name:String){
let x=description;
    let y=name;
    let benny= title+ &*x + &*y;
    // println!("⭐⭐⭐⭐⭐{}",data);

    will_win(benny);

   // let mut res=Response::new(data);
   //  HttpResponse::Ok().json(data)
}


pub async fn will_win(x:String)->HttpResponse{
    let mut handlebars = Handlebars::new();


    // Register the "index" template from a file
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

//let xy=String::from(benny);

    let benny_string= x.to_string();
    println!("⭐⭐⭐⭐⭐⭐ real star{}", benny_string);
    // Create a context with data
    let mut data = BTreeMap::new();
  //  data.insert("title", benny);
    data.insert("header", benny_string);
    data.insert("ji", "Jos buttler".parse().unwrap());
// data.insert("ok","England will win");
    // Render the template with the context
    let html = handlebars.render("index", &data).unwrap();

    // Return the HTML page as a response
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

