use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    dotenv().ok();
    let url: String = env::var("DATABASE_URL").expect("No database url set");
    let mgr: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(4)
        .build(mgr)
        .expect("Could not build connection pool")
}
