use env_logger::Builder;
use log::{error, info};
pub mod arc_clone;
pub mod asyncs;
pub mod boxs;
pub mod generics;
pub mod locks;
pub mod macros;
pub mod overload;
pub mod patterns;
pub mod simple_tests;
pub mod state_pattern;
pub mod threads;
pub mod unsafes;

pub fn lib() {
    // Initialize logger
    Builder::new().parse_filters("debug").init();
    simple_tests::hashmap_test();
    simple_tests::log_test();
    simple_tests::generics_test();
    boxs::drops::drop_test();
    boxs::linked_list::list_test();
    threads::move_test();
    threads::channel_test().unwrap_or_else(|e| {
        error!("{e}");
    });
    threads::mpsc_test().unwrap_or_else(|e| {
        error!("{e}");
    });
    locks::lock_test().unwrap_or_else(|e| {
        error!("{e}");
    });
    locks::lock_thread_test().unwrap_or_else(|e| {
        error!("{e}");
    });
    patterns::pattern_test();
    unsafes::rawpointer();
    unsafes::slices();
    unsafes::extern_test();
    macros::hrr_vec();
    asyncs::test();
    arc_clone::arc_clone_test();
    info!("Hello, world!");
}
