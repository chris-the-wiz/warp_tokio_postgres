use crate::{ data::*, DBPool, DBCon,  error, error::Error::*};
use mobc_postgres::{tokio_postgres, PgConnectionManager,};
use mobc::Pool;
use tokio_postgres::{Config, Error, NoTls, Row};
//use std::fs;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};


const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

const INIT_SQL: &str = "./src/db.sql";


pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
 //HandlerResult<DBPool> {
    let config = Config::from_str("postgres://postgres:postgres@127.0.0.1:5432/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager))
}



pub async fn get_db_con(db_pool: &DBPool) -> std::result::Result<DBCon, error::Error> {
    db_pool.get().await.map_err(DBPoolError)
}



//pub async fn init_db(db_pool: &DBPool) -> Result<(), Box<dyn std::error::Error>> {
pub async fn init_db(db_pool: &DBPool) -> Result<(), warp::Rejection> {
  
  let init_file = tokio::fs::read_to_string(INIT_SQL)
        .await
        .map_err(|io_error| warp::reject::custom(ReadFileError(io_error)))?;


  let con = get_db_con(db_pool).await?;
  con.batch_execute(init_file.as_str())
     .await
     .map_err(DBInitError)?;

  Ok(())
}



//////////////////
const TABLE: &str = "todo";
type DbResult<T> = std::result::Result<T, error::Error>;
//type HandlerResult<T> = std::result::Result<T, error::Error>;
/*
pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> DbResult<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
            .query_one(query.as_str(), &[&body.name])
            .await
            .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}
*/



pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> DbResult<Todo> {
        let con = get_db_con(db_pool).await?;
        let query = format!("INSERT INTO {} (name, checked) VALUES ($1, $2) RETURNING *", TABLE);
        let row = con
            .query_one(query.as_str(), &[&body.name, &body.checked])
            .await
            .map_err(DBQueryError)?;
        Ok(row_to_todo(&row))
    }

const SELECT_FIELDS: &str = "id, name, created_at, checked";

pub async fn fetch_todos(db_pool: &DBPool, search: Option<String>) -> DbResult<Vec<Todo>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
            Some(_) => "WHERE name like $1",
            None => "",
    };
    let query = format!(
            "SELECT {} FROM {} {} ORDER BY created_at DESC",
            SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
            Some(v) => con.query(query.as_str(), &[&v]).await,
            None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

pub async fn fetch_todo_by_id(db_pool: &DBPool, id: Option<i32>) -> DbResult<Vec<Todo>> {
        let con = get_db_con(db_pool).await?;
       

        let query = "SELECT id, name, created_at, checked FROM todo WHERE id = $1 ORDER BY created_at DESC";
        let rows = con.query(query, &[&Some(id)]).await?;

    
        Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
    }


pub async fn update_todo(db_pool: &DBPool, id: i32, body: TodoUpdateRequest) ->DbResult<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
            "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
            TABLE
    );
    let row = con
            .query_one(query.as_str(), &[&body.name, &body.checked, &id])
            .await
            .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &DBPool, id: i32) -> DbResult<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
            .await
            .map_err(DBQueryError)
}


fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
            id,
            name,
            created_at,
            checked,
    }
}


