use std::fs;
use handlebars::{Handlebars, Renderable};
use sqlx::postgres::PgPoolOptions;
use serde_json::json;
use sqlx::Error;
use serde::Serialize;


pub async fn finally() ->Result<(), sqlx::Error>{

    #[derive(sqlx::FromRow)]
    #[derive(Serialize)]
    struct User {
        title: String,
        description: String,
        name: String,
    }

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let query = sqlx::query_as::<_, User>("SELECT title,description ,name FROM posts");
    let users: Vec<User> = query.fetch_all(&pool).await?;


    for user in &users {
        println!("{}{}{}", user.title, user.description,user.name);
    }
    let mut handlebars = Handlebars::new();
    let template_source = "templates/index1.hbs" ;
    let template = handlebars::Template::compile(template_source).unwrap();

    // // Register the "index" template from a file
    // let index_template = fs::read_to_string("templates/index1.hbs").unwrap();
    // handlebars
    //     .register_template_string("index", &index_template)
    //     .unwrap();

  //  let html = template.render(&json!({"users": users})).unwrap();

    let context = json!({
    "users": users,
});
    let html = handlebars.render_template(template_source, &context).unwrap();


    // let template = handlebars::Template::compile(template_source).unwrap();
    // let html = template.render(&json!({"users": users})).unwrap();

Ok(())
}
