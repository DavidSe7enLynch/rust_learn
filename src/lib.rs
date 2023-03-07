use env_logger::Builder;
use log::{error, info};
mod boxs;
pub mod generics;
mod simple_tests;
mod state_pattern;

pub fn lib() {
    // Initialize logger
    Builder::new().parse_filters("debug").init();
    // simple_tests::hashmap_test();
    // simple_tests::log_test();
    simple_tests::generics_test();
    boxs::drops::drop_test();
    boxs::linked_list::list_test();
    info!("Hello, world!");
}
