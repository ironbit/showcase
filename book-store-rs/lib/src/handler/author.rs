// SPDX-License-Identifier: Apache-2.0
use {
    crate::{
        handler::{Context as HandlerContext, Query},
        model::{self, Author as ModelAuthor},
    },
    anyhow::{Error, Result},
    async_graphql::{Context, InputObject, Object, SimpleObject},
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone, Eq, PartialEq, InputObject, SimpleObject)]
#[graphql(input_name = "AuthorInput")]
pub struct Author {
    #[graphql(default_with = "Uuid::nil()")]
    pub id: Uuid,
    #[graphql(default = "")]
    pub first_name: String,
    #[graphql(default = "")]
    pub last_name: String,
}

impl Author {
    pub fn new(id: Uuid, first_name: String, last_name: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
        }
    }
}

impl From<&ModelAuthor> for Author {
    fn from(value: &ModelAuthor) -> Author {
        let id = if let Some(id) = value.id {
            id
        } else {
            Uuid::nil()
        };
        let first_name = if let Some(first_name) = &value.first_name {
            first_name.clone()
        } else {
            String::new()
        };
        let last_name = if let Some(last_name) = &value.last_name {
            last_name.clone()
        } else {
            String::new()
        };

        Self::new(id, first_name, last_name)
    }
}

#[allow(clippy::from_over_into)]
impl Into<ModelAuthor> for Author {
    fn into(self) -> ModelAuthor {
        let id = if self.id.is_nil() {
            None
        } else {
            Some(self.id)
        };
        let first_name = if self.first_name.is_empty() {
            None
        } else {
            Some(self.first_name)
        };
        let last_name = if self.last_name.is_empty() {
            None
        } else {
            Some(self.last_name)
        };

        ModelAuthor::new(id, first_name, last_name)
    }
}

#[Object]
impl Query {
    async fn author(&self, ctx: &Context<'_>, params: Author) -> Result<Option<Vec<Author>>> {
        // retrieve context
        let ctx = &ctx
            .data::<Arc<HandlerContext>>()
            .map_err(|_| Error::msg(""))?;

        // apply fetch
        let authors = model::fetch_author(&ctx.as_ref().database, params.into()).await?;

        // generate output
        match authors {
            None => Ok(None),
            Some(authors) => Ok(Some(authors.iter().map(Author::from).collect())),
        }
    }
}
