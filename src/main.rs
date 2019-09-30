#[macro_use]
extern crate actix;

use actix::*;

use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use std::sync::Arc;

#[macro_use]
extern crate juniper;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use crate::schema::{create_schema, Schema};

mod bookshelf;

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| Ok(HttpResponse::Ok().content_type("application/json").body(user)))
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    // let addr = bookshelf::Bookshelf.new().start();
    let addr = bookshelf::Bookshelf::new().start();
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap()
}
