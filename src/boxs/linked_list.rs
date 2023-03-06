use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn list_test() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}