use treeleaves::{
    config::{SharedConfigFile, TargetConfigFile},
    database::{connect_db_cluster, SharedConnection, TargetConnection},
    table::pop_files,
};

use std::{
    error::Error,
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Config
    let cfg_shared = SharedConfigFile::new().shared;

    let path = "test/target_data/.treeleaves/config.toml";
    let cfg_target = TargetConfigFile::new(path).target;

    // Connection
    let mut shared = SharedConnection(connect_db_cluster(&cfg_shared.url));
    let mut target = TargetConnection(connect_db_cluster(&cfg_target.url));
    let dir = std::env::args().nth(1).expect("Usage: files [dir]");
    println!("Generating metadata for {}", dir);
    let dir = Path::new(&dir);
    pop_files(&mut shared, &mut target, dir)?;
    Ok(())
}
