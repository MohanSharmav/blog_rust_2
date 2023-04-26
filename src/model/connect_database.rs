
pub async fn connect_database()
{



    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let  mut pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(100)
        .connect("&postgres://mohanvenkatesh:Msvmsd183!@localhost:5432/bloging")
        .await.expect("Unable to connect to Postgres");


}