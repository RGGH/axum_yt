mod routes;
pub use routes::routes_comp;



// Define the application state
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}
