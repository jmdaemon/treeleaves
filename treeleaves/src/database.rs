use diesel::prelude::*;

pub fn connect_db_cluster(url: &str) -> PgConnection {
    PgConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}
// Prevent users from using the wrong database cluster
pub struct SharedConnection(pub PgConnection);
pub struct TargetConnection(pub PgConnection);

#[macro_export]
macro_rules! batch_insert {
    ($con:expr, $table_name:expr, $table:expr, $records:expr) => {
        diesel::insert_into($table)
            .values($records)
            .execute($con)
            .expect(&format!("Could not fill {} table", $table_name));
        
    };
}
