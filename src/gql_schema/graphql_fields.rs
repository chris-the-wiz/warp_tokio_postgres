use async_graphql::*;
use crate::db_requests::store::Store;



#[Object]
impl Store {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn clients(&self) -> &Option<i32> { &self.clients }
}
