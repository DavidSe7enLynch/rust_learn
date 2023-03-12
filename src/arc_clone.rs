use std::sync::Arc;

// #[derive(Clone)]
struct Foo {
    data: i32,
}

pub fn arc_clone_test() {
    let arc1 = Arc::new(Foo { data: 42 });

    // Cloning with Arc::clone explicitly creates a new reference to the same data
    let arc2 = Arc::clone(&arc1);
    assert_eq!(arc1.data, arc2.data); // Both arc1 and arc2 point to the same data
    assert_eq!(Arc::strong_count(&arc1), 2); // The reference count is now 2
    assert_eq!(Arc::strong_count(&arc2), 2);

    let arc3 = arc1.clone();
    assert_eq!(arc1.data, arc3.data);
    assert_eq!(Arc::strong_count(&arc1), 3); // The reference count is 3
    assert_eq!(Arc::strong_count(&arc3), 3); // The reference count for arc3 is 3
}
