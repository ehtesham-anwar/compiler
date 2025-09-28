mod utils;
use std::collections::HashMap;
use utils::load_domains;

fn main() {
    println!("Hello, world!");
    let mut domain_map: HashMap<String, String> = HashMap::new();
    load_domains("config/domain.yml", &mut domain_map);
}
