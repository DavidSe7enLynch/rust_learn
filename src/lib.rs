use env_logger::Builder;
use log::{error, info};
mod simple_tests;
pub mod generics;


pub fn lib() {
    // Initialize logger
    Builder::new().parse_filters("debug").init();
    // simple_tests::hashmap_test();
    // simple_tests::log_test();
    simple_tests::generics_test();
    info!("Hello, world!");
}