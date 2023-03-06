use std::rc::Rc;

use log::info;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn list_test() {
    let a = Rc::new(Cons(1, Rc::new(Cons(0, Rc::new(Nil)))));
    info!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(2, Rc::clone(&a));
    info!("count after creating a = {}", Rc::strong_count(&a));
    let c = Cons(3, Rc::clone(&a));
    info!("count after creating a = {}", Rc::strong_count(&a));
}
