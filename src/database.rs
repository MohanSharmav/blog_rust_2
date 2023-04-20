
use sqlx::{Error, PgPool, Pool, Postgres, Row};
use tokio::select;

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

async fn select(pool: &PgPool)-> Result<(),Error>{
    // let rows = sqlx::query("SELECT name FROM categories")
    //     .map(|row:sqlx::postgres::PgRow|{
    //          let a:String=row.get("name");
    //       println!("{:?}",rows)
    //     })
    //     .fetch_all(pool)
    //     .await?;
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(pool)
        .await?;

    for row in rows{
        let name: String=row.get("name");
        println!("name is : {}",name)
    }
    Ok(())
}