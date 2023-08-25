use crate::{data::*, db, DBPool, DBCon,  error, error::Error::*,};
use warp::{http::StatusCode, reject, Reply, Rejection,reply::json};
use serde::Deserialize;

type HandlerResult<T> = std::result::Result<T, Rejection>;




pub async fn health_handler(db_pool: DBPool) ->HandlerResult<impl Reply> {
    let db = db::get_db_con(&db_pool)
            .await
            .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn create_todo_handler(
        body: TodoRequest, 
       // body: warp::hyper::body::Bytes,
        db_pool: DBPool) ->HandlerResult<impl Reply> {

      //  let _tt = 10;
        // json gets truncated if parsed directly
     //   let param = String::from_utf8_lossy(&body).to_string();
      //  println!("param: {}", param);
      //  let json_request:crate::data::TodoRequest = serde_json::from_str(param.as_str()).unwrap_or_else(|err| { panic!("error parsing json: {}", err) });
            println!("{:?}", body.to_string());

        Ok(json(&TodoResponse::of(
            db::create_todo(&db_pool, body)
                .await
                .map_err(|e| reject::custom(e))?,
        )))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    name: Option<String>,
    id: Option<i32>
}


#[derive(Debug)]
struct DivideByZero;

impl reject::Reject for DivideByZero {}


#[derive(Debug)]
struct InvalidParam;

impl reject::Reject for InvalidParam{}


pub async fn list_todos_handler(query: SearchQuery, db_pool: DBPool) -> HandlerResult<impl Reply> {
    if let Some(id_value) = &query.id {
        
        println!("query.id has a value: {}", id_value);
        let todos = db::fetch_todo_by_id(&db_pool, query.id)
        .await
        .map_err(|e| reject::custom(e))?;

        Ok(json::<Vec<_>>(&todos.into_iter().map(|t| TodoResponse::of(t)).collect(),))

    } else if let Some(name_value)= &query.name {
        
        println!("query.name has a value: {}", name_value);
        let todos = db::fetch_todos(&db_pool, query.name)
        .await
        .map_err(|e| reject::custom(e))?;

        Ok(json::<Vec<_>>(&todos.into_iter().map(|t| TodoResponse::of(t)).collect(),))

    } else {
     
        let todos = db::fetch_todos(&db_pool, None)
        .await
        .map_err(|e| reject::custom(e))?;

        Ok(json::<Vec<_>>(&todos.into_iter().map(|t| TodoResponse::of(t)).collect(),))
        

     
   //  Err(reject::reject()) //weird
    //Err(reject::custom(InvalidParam)) // how to return InvalidQuery??
    //Err(reject::custom(warp::reject::InvalidQuery))
   // Err(warp::reject::invalid_query )
    }
    
   
  
}


/* 
pub async fn list_single_todo_handler(id: i32, 
    query: SearchQuery,
      db_pool: DBPool) -> HandlerResult<impl Reply> {
    let todos = db::fetch_todo_by_id(&db_pool, query.id)
            .await
            .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
            &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
    ))
}
*/


pub async fn update_todo_handler(
    id: i32,
    //body: TodoUpdateRequest, no direct conversion / doesnt work
    body :  warp::hyper::body::Bytes,
    db_pool: DBPool,
) -> HandlerResult<impl Reply> {
   
    let param = String::from_utf8_lossy(&body).to_string();
    let json_request:crate::data::TodoUpdateRequest = serde_json::from_str(param.as_str()).unwrap_or_else(|err| { panic!("error parsing json: {}", err) });
    

    Ok(
        json(&TodoResponse::of(
        db::update_todo(&db_pool, id,json_request)
            .await
            .map_err(|e| reject::custom(e))?,
        ))
    )
}

pub async fn delete_todo_handler(id: i32, db_pool: DBPool) -> HandlerResult<impl Reply> {
    db::delete_todo(&db_pool, id)
            .await
            .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}