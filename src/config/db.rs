use sqlx::PgPool;

pub fn create_pool(database_url: &str) -> PgPool {
    PgPool::connect_lazy(database_url).expect("Failed to create database pool")
}