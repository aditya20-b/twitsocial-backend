use regex::Regex;
use crate::models::AllowedMethods;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

// This type just definites the pool object of the connections
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

// This type defines the actual connection objects generated from the pool
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


fn establish_connection(database_url: &str) -> PgPooledConnection {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
        .get()
        .expect("Failed to get connection from pool.")
}

impl AllowedMethods {
    pub fn new(regex: String, methods: Vec<String>) -> Self {
        Self {
            pattern: Regex::new(&regex).unwrap(),
            methods,
        }
    }
}
