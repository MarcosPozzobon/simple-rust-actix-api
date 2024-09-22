use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_environment = std::env::var("DATABASE_URL_STOCK").expect("Database URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_environment)
        .await
        .expect("Failed to connect to postgres database");

    let check_migrate = sqlx::migrate!("./src/database/postgres_connection/migrations")
        .run(&pool)
        .await;
    match check_migrate{
        Ok(_) => println!("Migrations ran sucessfully!"),
        Err(e) => println!("Error while running migrations {}", e)
    }
    pool
}