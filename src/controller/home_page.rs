#[warn(unused_imports)]
use std::collections::HashMap;
use std::fs;
use actix_web::{HttpResponse};
use warp::reply::html;
use crate::model::Homepage_query::{get_all_posts, posts};
pub async fn default_page() -> HttpResponse
{


    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

   let mut data =HashMap::new();
    data.insert("name","boss");
    let html = handlebars.render("index", &data).unwrap();

    //test  start
//test end
    HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(html)



}

pub fn get_all_posts_in_home_page(posts:posts){

}
