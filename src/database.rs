use actix_web::{HttpRequest, HttpResponse, Responder};
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
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
        println!("{}", title);
        println!("{}", description);
        println!("{}", name);
        get_data(title, description, name).await;

    }

    Ok(())
}

pub async fn get_data(title:String,description:String,name:String)-> impl Responder{
let x=description;
    let y=name;
    let data= title+ &*x + &*y;
    println!("⭐⭐⭐⭐⭐{}",data);

    HttpResponse::Ok().json(data)
}