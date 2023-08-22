use treeleaves::{
    database::{DatabaseClusterType, connect_db_cluster, SharedConnection, TargetConnection},
    table::pop_files,
};

use std::{
    error::Error,
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut shared = SharedConnection(connect_db_cluster(DatabaseClusterType::SHARED));
    let mut target = TargetConnection(connect_db_cluster(DatabaseClusterType::TARGET));
    let dir = std::env::args().nth(1).expect("Usage: files [dir]");
    println!("Generating metadata for {}", dir);
    let dir = Path::new(&dir);
    pop_files(&mut shared, &mut target, dir)?;
    Ok(())
}
