#[macro_use]
mod common;

#[cfg(test)]
mod sqlite_store_tests {
    use axum::Router;
    use tower_sessions::SessionManagerLayer;
    use tower_sessions_sqlx_store_chrono::{sqlx::SqlitePool, SqliteStore};

    use crate::common::build_app;

    async fn app(max_age: Option<Duration>) -> Router {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let session_store = SqliteStore::new(pool);
        session_store.migrate().await.unwrap();
        let session_manager = SessionManagerLayer::new(session_store).with_secure(true);

        build_app(session_manager, max_age)
    }

    route_tests!(app);
}

#[cfg(test)]
mod postgres_store_tests {
    use axum::Router;
    use tower_sessions::SessionManagerLayer;
    use tower_sessions_sqlx_store_chrono::{sqlx::PgPool, PostgresStore};

    use crate::common::build_app;

    async fn app(max_age: Option<Duration>) -> Router {
        let database_url = std::option_env!("POSTGRES_URL").unwrap();
        let pool = PgPool::connect(database_url).await.unwrap();
        let session_store = PostgresStore::new(pool);
        session_store.migrate().await.unwrap();
        let session_manager = SessionManagerLayer::new(session_store).with_secure(true);

        build_app(session_manager, max_age)
    }

    route_tests!(app);
}

#[cfg(test)]
mod mysql_store_tests {
    use axum::Router;
    use tower_sessions::SessionManagerLayer;
    use tower_sessions_sqlx_store_chrono::{sqlx::MySqlPool, MySqlStore};

    use crate::common::build_app;

    async fn app(max_age: Option<Duration>) -> Router {
        let database_url = std::option_env!("MYSQL_URL").unwrap();

        let pool = MySqlPool::connect(database_url).await.unwrap();
        let session_store = MySqlStore::new(pool);
        session_store.migrate().await.unwrap();
        let session_manager = SessionManagerLayer::new(session_store).with_secure(true);

        build_app(session_manager, max_age)
    }

    route_tests!(app);
}
