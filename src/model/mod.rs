use async_graphql::{Context, Object, Schema, EmptyMutation, EmptySubscription};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }
}