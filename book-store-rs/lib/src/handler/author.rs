// SPDX-License-Identifier: Apache-2.0
use {
    crate::{
        handler::{Context as HandlerContext, Query},
        model::{self, Author as ModelAuthor},
    },
    anyhow::{Error, Result},
    async_graphql::{Context, Object, SimpleObject},
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(SimpleObject)]
pub struct Author {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl TryFrom<ModelAuthor> for Author {
    type Error = Error;

    fn try_from(value: ModelAuthor) -> Result<Author> {
        Ok(Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
        })
    }
}

#[Object]
impl Query {
    async fn author(&self, ctx: &Context<'_>, id: Uuid) -> Result<Author> {
        let ctx = &ctx
            .data::<Arc<HandlerContext>>()
            .map_err(|_| Error::msg(""))?;
        let model_author = model::fetch_author(&ctx.as_ref().database, id).await?;
        let author = Author::try_from(model_author)?;
        Ok(author)
    }
}
