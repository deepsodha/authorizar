use once_cell::sync::OnceCell;
use sqlx::migrate::MigrateDatabase;
use sqlx::{pool::PoolConnection, Pool, Sqlite, SqlitePool};

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

#[inline]
pub async fn get_conn() -> PoolConnection<Sqlite> {
    unsafe {
        return POOL
            .get_unchecked()
            .acquire()
            .await
            .expect("Unable to acquire DB connection");
    }
}

pub async fn initialize(db_url: &str) {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Creating new db"),
            Err(error) => panic!("Failed to create a database: {}", error),
        }
    } else {
        println!("Database already exist.")
    }

    let _ = match SqlitePool::connect(db_url).await {
        Ok(p) => POOL.set(p),
        _ => panic!("Failed to initialize DB pool"),
    };
}
