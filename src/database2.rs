use std::collections::BTreeMap;
use std::fs;
use handlebars::{Handlebars, Renderable};
use sqlx::postgres::PgPoolOptions;
use serde_json::json;
use sqlx::Error;
use serde::Serialize;

#[derive(sqlx::FromRow)]
#[derive(Serialize)]
pub struct User {
    title: String,
    description: String,
    name: String,
}
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use actix_web::ResponseError;
// use serde_json::Value::String;
use warp::trace::named;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.title)
    }
}
//


//
// pub async fn one_final_page( users: Vec<User>){
//
//     let mut handlebars = Handlebars::new();
//     let template_source = "templates/index1.hbs" ;
//     let template = handlebars::Template::compile(template_source);
//
//     // // Register the "index" template from a file
//     // let index_template = fs::read_to_string("templates/index1.hbs").unwrap();
//     // handlebars
//     //     .register_template_string("index", &index_template)
//     //     .unwrap();
//
//     //  let html = template.render(&json!({"users": users})).unwrap();
//
//     let context = json!({
//     "users": users,
// });
//     let html = handlebars.render_template(template_source, &context);
//
// }


pub async fn finally() ->Result<(),Error>{

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

    let mut data = BTreeMap::new();

// let c=users.to_string().unwrap;
//     println!("{}",c);
    for user in &users {
     // let m: String =users.get(name).unwrap();
     //    let n:String= users.get(title).unwrap();
     //    let o:String = users.get(description).unwrap();
      //  println!("2022 winners{}",m);
data.insert("jof",&user.name);
        println!("ggggggggggg {}",user.title);
        println!("ben sotkes");
        println!("we are england's ⭐⭐⭐⭐⭐{}{}{}", user.title, user.description,user.name);
    }


    //one_final_page(users).await;


    // let template = handlebars::Template::compile(template_source).unwrap();
    // let html = template.render(&json!({"users": users})).unwrap();

Ok(())
}
