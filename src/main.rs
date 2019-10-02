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
    // st: web::Data<Arc<Schema>>,
    state: web::Data<State>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        return match state.get_ref() {
            State { schema, addr } => {
                let ctx = schema::Context { addr: addr.clone() };
                let res = data.execute(&schema, &ctx);
                Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
            }
        };
        // let (schema, addr) = state;
        // let ctx = schema::Context { addr };
        // let ctx = schema::Context {
        //     addr: state.addr.clone(),
        // };
        // let res = data.execute(&st, &ctx);
        // let res = data.execute(&(state.schema), &ctx);
        // let res = data.execute(&schema, &ctx);
        // Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| Ok(HttpResponse::Ok().content_type("application/json").body(user)))
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct State {
    schema: Arc<Schema>,
    addr: actix::Addr<bookshelf::Bookshelf>,
}

fn main() -> std::io::Result<()> {
    let sys = actix::System::new("sync-marker");

    let addr = bookshelf::Bookshelf::new().start();
    let schema = std::sync::Arc::new(create_schema());
    // let state = State {
    //     schema: schema.clone(),
    //     addr: addr.clone(),
    // };
    HttpServer::new(move || {
        App::new()
            // .data(schema.clone())
            .data(State {
                schema: schema.clone(),
                addr: addr.clone(),
            })
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .start();
    sys.run()
}