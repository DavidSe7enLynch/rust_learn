use env_logger::Builder;

mod generics;
mod simple_tests;

fn main() {
    // Initialize logger
    Builder::new().parse_filters("debug").init();
    // simple_tests::hashmap_test();
    // simple_tests::log_test();
    simple_tests::generics_test();
    println!("Hello, world!");
}
