// SPDX-License-Identifier: Apache-2.0
use {
    crate::{
        config,
        handler::{self, Schema as HandlerSchema},
        model::Database,
    },
    anyhow::Result,
    async_graphql::http::{playground_source, GraphQLPlaygroundConfig},
    async_graphql_axum::{GraphQLRequest, GraphQLResponse},
    axum::{
        extract::Extension,
        response::{self, IntoResponse},
        routing::get,
        Router,
    },
};

#[derive(Default)]
pub struct Server;

impl Server {
    #[allow(clippy::pedantic)]
    pub async fn run(&self) -> Result<()> {
        let config = config::Config::new()?;
        let database = Database::new(config).await?;
        let schema = handler::create_schema(database);

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .layer(Extension(schema));

        axum::Server::bind(&"0.0.0.0:8000".parse()?)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

async fn graphql_handler(schema: Extension<HandlerSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    let data = async { playground_source(GraphQLPlaygroundConfig::new("/")) }.await;
    response::Html(data)
}
