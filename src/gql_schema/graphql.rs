use async_graphql::*;
use sqlx::PgPool;
use crate::db_requests::store::Store;

#[derive(Default)]
pub struct Query;
#[Object]
impl Query {
    async fn get_all_stores(&self, ctx: &Context<'_>) -> Result<Vec<Store>, Error> {
        let stores = Store::get_all_stores(&ctx.data::<PgPool>().unwrap()).await?;
        Ok(stores)
    }
}
#[derive(Default)]
pub struct Mutation;
#[Object]
impl Mutation {
    async fn create_store(&self, ctx: &Context<'_>, name : String, clients: Option<i32>) -> Result<Store, Error> {
        let new_store = Store {
            id: 10,
            name,
            clients,
        };

        let created_store_entity =
            Store::create_store(&ctx.data::<PgPool>().unwrap(), new_store).await?;

        Ok(created_store_entity)
    }
}

