use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn list_test() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn mybox_test() {
    let mybox = MyBox::new(1);
    assert_eq!(1, *mybox);
}
