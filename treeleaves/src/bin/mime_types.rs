use treeleaves::{
    config::SharedConfigFile,
    database::{connect_db_cluster, SharedConnection},
    table::pop_mime_types,
};

fn main() {
    let shared = SharedConfigFile::new().shared;
    let mut con = SharedConnection(connect_db_cluster(&shared.url));
    pop_mime_types(&mut con);
}
