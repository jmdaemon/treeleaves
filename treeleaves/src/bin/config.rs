use treeleaves::config::{SharedConfigFile, TargetConfigFile};

fn main() {
    let shared = SharedConfigFile::new().shared;
    println!("{}", shared.url);

    let path = "test/target_data/.treeleaves/config.toml";
    let target = TargetConfigFile::new(path).target;
    println!("{}", target.url);
}
