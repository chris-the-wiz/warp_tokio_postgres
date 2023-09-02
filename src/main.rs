use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection, http::Response};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing;
use sqlx::PgPool;

use async_graphql::{Schema, Request, http::playground_source, http::GraphQLPlaygroundConfig};

mod data;
mod db;
mod error;
mod handler;

mod gql_schema;
mod db_requests;

#[cfg(test)]
pub mod tests;  


type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;



fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}


pub async  fn create_connection_pool() -> PgPool {
    PgPool::connect("postgres://postgres:postgres@127.0.0.1:5432/postgres").await.unwrap()
}


#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");
    
    db::init_db(&db_pool)
    .await
    .expect("database can not be initialized");

    let health_route = 
     warp::path!("health")
    .and(with_db(db_pool.clone()))
    .and_then(handler::health_handler);
   


    let todo = warp::path("todo");

    let todo_routes =
    todo
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_todos_handler)
    .or(todo
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handler::create_todo_handler))
    .or(todo
        .and(warp::put())
        .and(warp::path::param())
        //.and(warp::body::json()) /// for some reason it does not work directly
        .and(warp::body::bytes())
        .and(with_db(db_pool.clone()))
        .and_then(handler::update_todo_handler))
    .or(todo
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and_then(handler::delete_todo_handler));




    let schema = gql_schema::build_schema(create_connection_pool().await).finish();

    let store = warp::path("store");

    let store_routes = 
    store
            .and(warp::post())
            .and(async_graphql_warp::graphql(schema))
            .and_then( | (schema, request): (Schema<_,_,_>, Request)| async move {
            let response = schema
                .execute(request).await;

            Ok::<_, Infallible>(async_graphql_warp::GraphQLResponse::from(response))
        });

    
    //GraphQL Playground
    let graphql_playground =  warp::path("graphql").map(||{
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/store")))
    });




    let routes =
     health_route
    .or(todo_routes)
    .or(store_routes)
    .or(graphql_playground)
    .with(warp::cors().allow_any_origin().allow_methods(vec!["GET", "POST", "DELETE", "PUT"]))
    .recover(error::handle_rejection);
     





   
async fn goodbye( body: warp::hyper::body::Bytes)-> Result<impl warp::Reply, Infallible> {
//async fn goodbye()-> Result<impl warp::Reply, Infallible> {

    let param = String::from_utf8_lossy(&body).to_string();
    println!("param: {}", param);
    let jsonresp:data::TodoUpdateRequest = serde_json::from_str(param.as_str()).unwrap_or_else(|err| { panic!("error parsing json: {}", err) });
  
    let _k= 5;
    tracing::info!("saying goodbye...");

    Ok(jsonresp.to_string())
    //Ok("Hello, async Warp!")
  }
  

   let goodbye = warp::path("goodbye")
   .and(warp::put())
   //.and(warp::path::param())
   .and(warp::body::bytes())
   .and_then(goodbye);


  //   let test_route  = put_route
    


    // warp::serve(goodbye).run(([127, 0, 0, 1], 8000)).await;
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}