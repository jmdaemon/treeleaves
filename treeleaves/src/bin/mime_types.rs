use treeleaves::{
    database::{DatabaseClusterType, connect_db_cluster, SharedConnection},
    table::pop_mime_types,
};

fn main() {
    let mut con = SharedConnection(connect_db_cluster(DatabaseClusterType::SHARED));
    pop_mime_types(&mut con);
}
