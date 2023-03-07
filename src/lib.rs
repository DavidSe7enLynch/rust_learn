use env_logger::Builder;
use log::{error, info};
mod boxs;
pub mod generics;
mod locks;
mod simple_tests;
mod state_pattern;
mod threads;

pub fn lib() {
    // Initialize logger
    Builder::new().parse_filters("debug").init();
    // simple_tests::hashmap_test();
    // simple_tests::log_test();
    // simple_tests::generics_test();
    // boxs::drops::drop_test();
    // boxs::linked_list::list_test();
    // threads::move_test();
    // threads::channel_test().unwrap_or_else(|e| {
    //     error!("{e}");
    // });
    // threads::mpsc_test().unwrap_or_else(|e| {
    //     error!("{e}");
    // });
    locks::lock_test().unwrap_or_else(|e| {
        error!("{e}");
    });
    info!("Hello, world!");
}
