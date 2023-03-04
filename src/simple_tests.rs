use crate::generics;
use log::{error, info};
use std::collections::HashMap;

pub fn hashmap_test() {
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
    // Log some messages
    error!("This is an error message");
    info!("This is an info message");
}

pub fn generics_test() {
    let mut point = generics::Point::new(10, 20);
    let x = point.x();
    *x = 1;
    info!("point.x = {}", point.x());
    let larger = point.larger();
    info!("larger = {}", larger);
}
