use sqlx::{Error, PgPool};

#[derive(Debug, Clone)]
pub struct Store {
 pub id: i32,
 pub name: String,
 pub clients: Option<i32>,
}

impl Store {
 pub async fn create_store(pool: &PgPool, store: Store) -> Result<Store, Error> {
   // sqlx::query!("INSERT INTO stores VALUES ( $2, $3)",  store.name, store.clients)
    //.execute(pool)
    //.await?;

    let resp: Store = sqlx::query_as!(
        Store,
        "INSERT INTO stores (name, clients) VALUES ($1, $2) RETURNING id, name, clients",
        store.name,
        store.clients
    )
    .fetch_one(pool)
    .await?;



     //sqlx::query!("INSERT INTO stores VALUES ($1, $2, $3)", store.id, store.name, store.clients)
     //    .execute(pool)
     //    .await?;
    //println!("{resp}");

    Ok(resp) // id is wrong here
     //Ok(())
 }

 pub async fn get_all_stores(pool: &PgPool) -> Result<Vec<Store>, Error> {
     let stores = sqlx::query_as!(Store, "SELECT * FROM stores")
         .fetch_all(pool)
         .await?;

     Ok(stores)
 }

}