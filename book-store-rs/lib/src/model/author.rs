// SPDX-License-Identifier: Apache-2.0
use {
    super::Database,
    anyhow::Result,
    db_entity::authors::{Column, Entity as EntityAuthor, Model as ModelAuthor},
    sea_orm::{query::Condition, ColumnTrait, EntityTrait, QueryFilter},
    uuid::Uuid,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Author {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Author {
    pub fn new(id: Option<Uuid>, first_name: Option<String>, last_name: Option<String>) -> Self {
        Self {
            id,
            first_name,
            last_name,
        }
    }
}

impl From<&ModelAuthor> for Author {
    fn from(value: &ModelAuthor) -> Self {
        let id = if value.id.is_nil() {
            None
        } else {
            Some(value.id)
        };
        let first_name = if value.first_name.is_empty() {
            None
        } else {
            Some(value.first_name.clone())
        };
        let last_name = if value.last_name.is_empty() {
            None
        } else {
            Some(value.last_name.clone())
        };

        Self::new(id, first_name, last_name)
    }
}

pub async fn fetch(db: &Database, author: Author) -> Result<Option<Vec<Author>>> {
    if let Some(id) = author.id {
        return fetch_by_id(db, id).await;
    }

    let first_name = match author.first_name {
        Some(first_name) => first_name,
        None => String::new(),
    };
    let last_name = match author.last_name {
        Some(last_name) => last_name,
        None => String::new(),
    };

    if !first_name.is_empty() && !last_name.is_empty() {
        return fetch_by_name(db, first_name, last_name).await;
    }
    if !first_name.is_empty() {
        return fetch_by_first_name(db, first_name).await;
    }
    if !last_name.is_empty() {
        return fetch_by_last_name(db, last_name).await;
    }

    fetch_all(db).await
}

async fn fetch_all(db: &Database) -> Result<Option<Vec<Author>>> {
    let entities = EntityAuthor::find().all(&db.connection).await?;
    parse_authors(entities).await
}

async fn fetch_by_id(db: &Database, id: Uuid) -> Result<Option<Vec<Author>>> {
    let entity = EntityAuthor::find_by_id(id).one(&db.connection).await?;
    match entity {
        Some(author) => Ok(Some(vec![Author::from(&author)])),
        None => Ok(None),
    }
}

async fn fetch_by_name(
    db: &Database,
    first_name: String,
    last_name: String,
) -> Result<Option<Vec<Author>>> {
    let entities = EntityAuthor::find()
        .filter(
            Condition::any()
                .add(Column::FirstName.eq(first_name))
                .add(Column::LastName.eq(last_name)),
        )
        .all(&db.connection)
        .await?;

    parse_authors(entities).await
}

async fn fetch_by_first_name(db: &Database, first_name: String) -> Result<Option<Vec<Author>>> {
    let entities = EntityAuthor::find()
        .filter(Column::FirstName.eq(first_name))
        .all(&db.connection)
        .await?;

    parse_authors(entities).await
}

async fn fetch_by_last_name(db: &Database, last_name: String) -> Result<Option<Vec<Author>>> {
    let entities = EntityAuthor::find()
        .filter(Column::LastName.eq(last_name))
        .all(&db.connection)
        .await?;

    parse_authors(entities).await
}

async fn parse_authors(entities: Vec<ModelAuthor>) -> Result<Option<Vec<Author>>> {
    async {
        let authors: Vec<Author> = entities.iter().map(Author::from).collect();
        Ok(Some(authors))
    }
    .await
}

#[cfg(test)]
mod tests {
    use {
        super::{fetch, Author, Database},
        anyhow::Result,
        db_entity::authors::Model as ModelAuthor,
        sea_orm::{DatabaseBackend, MockDatabase, Transaction},
        uuid::Uuid,
    };

    #[tokio::test]
    async fn fetch_by_id() -> Result<()> {
        // create database
        let db = create_mock_database().await;

        // execute fetch
        let id = Uuid::parse_str("f33711d9-03f9-4f53-9e36-1242408b9b32").unwrap();
        let author = Author::new(Some(id), None, None);
        fetch(&db, author).await?;

        // expected transaction
        let txn = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            concat!(
                r#"SELECT "authors"."id", "authors"."first_name", "authors"."last_name" "#,
                r#"FROM "authors" "#,
                r#"WHERE "authors"."id" = $1 "#,
                r#"LIMIT $2"#,
            ),
            vec![id.into(), 1u64.into()],
        );

        // verify
        assert_eq!(db.connection.into_transaction_log(), vec![txn]);

        // done
        Ok(())
    }

    #[tokio::test]
    async fn fetch_by_name() -> Result<()> {
        // create database
        let db = create_mock_database().await;

        // execute fetch
        let first_name = String::from("name");
        let last_name = String::from("name");
        let author = Author::new(None, Some(first_name.clone()), Some(last_name.clone()));
        fetch(&db, author).await?;

        // expected transaction
        let txn = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            concat!(
                r#"SELECT "authors"."id", "authors"."first_name", "authors"."last_name" "#,
                r#"FROM "authors" "#,
                r#"WHERE "authors"."first_name" = $1 OR "authors"."last_name" = $2"#,
            ),
            vec![first_name.into(), last_name.into()],
        );

        // verify
        assert_eq!(db.connection.into_transaction_log(), vec![txn]);

        // done
        Ok(())
    }

    #[tokio::test]
    async fn fetch_by_first_name() -> Result<()> {
        // create database
        let db = create_mock_database().await;

        // execute fetch
        let name = String::from("name");
        let author = Author::new(None, Some(name.clone()), None);
        fetch(&db, author).await?;

        // expected transaction
        let txn = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            concat!(
                r#"SELECT "authors"."id", "authors"."first_name", "authors"."last_name" "#,
                r#"FROM "authors" "#,
                r#"WHERE "authors"."first_name" = $1"#,
            ),
            vec![name.into()],
        );

        // verify
        assert_eq!(db.connection.into_transaction_log(), vec![txn]);

        // done
        Ok(())
    }

    #[tokio::test]
    async fn fetch_by_last_name() -> Result<()> {
        // create database
        let db = create_mock_database().await;

        // execute fetch
        let name = String::from("name");
        let author = Author::new(None, None, Some(name.clone()));
        fetch(&db, author).await?;

        // expected transaction
        let txn = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            concat!(
                r#"SELECT "authors"."id", "authors"."first_name", "authors"."last_name" "#,
                r#"FROM "authors" "#,
                r#"WHERE "authors"."last_name" = $1"#,
            ),
            vec![name.into()],
        );

        // verify
        assert_eq!(db.connection.into_transaction_log(), vec![txn]);

        // done
        Ok(())
    }

    #[tokio::test]
    async fn fetch_all_authors() -> Result<()> {
        // create database
        let db = create_mock_database().await;

        // execute fetch
        let author = Author::new(None, None, None);
        fetch(&db, author).await?;

        // expected transaction
        let txn = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            concat!(
                r#"SELECT "authors"."id", "authors"."first_name", "authors"."last_name" "#,
                r#"FROM "authors""#,
            ),
            vec![],
        );

        // verify
        assert_eq!(db.connection.into_transaction_log(), vec![txn]);

        // done
        Ok(())
    }

    async fn create_mock_database() -> Database {
        async {
            let mock_data = vec![ModelAuthor {
                id: Uuid::nil(),
                first_name: String::new(),
                last_name: String::new(),
            }];

            let connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results(vec![mock_data])
                .into_connection();

            Database { connection }
        }
        .await
    }
}
