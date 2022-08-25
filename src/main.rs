// #![deny(unused_imports, dead_code, unused_variables)]

use std::env;
use std::sync::Arc;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper_codegen;

extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{
    guard, http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Error
};

use dotenv::dotenv;
use juniper_actix::{graphiql_handler, graphql_handler};
use log::info;

mod context;
mod database;
mod graphql;
mod models;
mod schema;

use crate::context::GraphQLContext;
use crate::database::get_pool;
use crate::graphql::{create_schema, Schema};

async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

async fn graphql(
    req: HttpRequest,
    ctx: web::Data<Arc<GraphQLContext>>,
    payload: web::Payload,
    schema: web::Data<Arc<Schema>>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &ctx, req, payload).await
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    env::set_var("RUST_LOG", "debug,actix_web=debug");
    let server_address: String = env::var("SERVER").expect("No database url set");
    env_logger::init();

    // Create the auto managed database pool
    let context = Arc::new(GraphQLContext { pool: get_pool().clone() });
    let schema = Arc::new(create_schema());

    info!("Started backend server: {}:5050", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(context.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind((server_address, 5050))
    .unwrap()
    .run()
    .await
}
