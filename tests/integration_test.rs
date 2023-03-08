use library;

#[test]
fn test_point() {
    use library::generics::Point;
    let _point = Point::new(1, 2);
    assert_ne!(1, 0);
}
