#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}
