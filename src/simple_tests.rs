pub fn hashmap_test() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // let score = map.get(&word).copied().unwrap_or(0);
    }

    println!("{:?}", map);
}

pub fn log_test() {
    use env_logger::Builder;
    use log::{error, info};

    // Initialize logger
    Builder::new().parse_filters("debug").init();

    // Log some messages
    error!("This is an error message");
    info!("This is an info message");
}
