// SPDX-License-Identifier: Apache-2.0
use {
    crate::{
        handler::{Context, Query},
        model::Database,
    },
    async_graphql::{EmptyMutation, EmptySubscription, Schema},
    std::sync::Arc,
};

pub type Handler = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create(database: Database) -> Handler {
    let context = Arc::new(Context::new(database));

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(context)
        .finish()
}
