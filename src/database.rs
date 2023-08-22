use diesel::prelude::*;

// Database Cluster URLs
pub const DB_SHARED_URL: &str = include_str!("url.shared");
pub const DB_TARGET_URL: &str = include_str!("url.target");

// Data
pub enum DatabaseClusterType {
    SHARED,
    TARGET
}

pub fn connect_db_cluster(cluster_type: DatabaseClusterType) -> PgConnection {
    let url = match cluster_type {
        DatabaseClusterType::SHARED => DB_SHARED_URL,
        DatabaseClusterType::TARGET => DB_TARGET_URL
    };
    PgConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

#[macro_export]
macro_rules! batch_insert {
    ($con:expr, $table_name:expr, $table:expr, $records:expr) => {
        diesel::insert_into($table)
            .values($records)
            .execute($con)
            .expect(&format!("Could not fill {} table", $table_name));
        
    };
}
