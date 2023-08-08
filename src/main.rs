use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool};

use another_paginate::paginate_database::{DataPages, Page, Pages};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct Blogs {
    pub post_id: i32,
    pub blog_id: i32,
    pub category_id: i32,
}

#[derive(Debug)]
pub struct BlogDB {
    pool: Pool<sqlx::Postgres>,
}

impl DataPages for BlogDB {
    #[tokio::main]
    async fn count_length(&self) -> usize {
        #[derive(Debug, FromRow, Clone)]
        pub struct Count {
            count: Option<i64>,
        }
        let result = sqlx::query_as!(Count, "select count(*) from blogs;")
            .fetch_all(&self.pool)
            .await
            .expect("Unable to connect to Postgres");

        match result[0].count {
            Some(v) => v as usize,
            None => 0,
        }
    }

    #[tokio::main]
    async fn query_pages(
        &self,
        start: usize,
        limit: usize,
    ) -> Result<std::string::String, serde_json::Error> {
        let result = sqlx::query_as!(Blogs, "select * from blogs;")
            .fetch_all(&self.pool)
            .await
            .expect("Unable to execute query");
        serde_json::to_string(&result)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let mut pages: Pages<BlogDB> = Pages::new(2, 1, BlogDB { pool: pool });
    // print!("{:#?}", pages);
    print!("{:#?}", pages.next());
}
