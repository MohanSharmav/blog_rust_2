
use sqlx::{Error, Row};
use sqlx::postgres::PgPoolOptions;



pub(crate) async fn selecting() -> Result<(),Error>{
    // let rows = sqlx::query("SELECT name FROM categories")
    //     .map(|row:sqlx::postgres::PgRow|{
    //          let a:String=row.get("name");
    //       println!("{:?}",rows)
    //     })
    //     .fetch_all(pool)
    //     .await?;


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


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


// async fn select_all_from_table(pool: &Pool<Postgres>) -> anyhow::Result<Vec<(i32, String)>> {
//      // let rows = sqlx::query("SELECT title,description,name FROM posts")
//      //     .fetch_all(pool)
//      //     .await?;
//      let rows = sqlx::query("SELECT name FROM categories")
//          .fetch_all(pool)
//          .await?;
//      let results = rows
//          .into_iter()
//          .map(|row| (row.get(0), row.get(1)))
//          .collect();
//
//      Ok(results)
// }