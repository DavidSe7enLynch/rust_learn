use std::{cell::RefCell, rc::Rc};

use log::info;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn list_test() {
    let val = Rc::new(RefCell::new(0));
    let a = Rc::new(Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(Rc::clone(&val), Rc::new(Nil))),
    ));
    info!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    info!("count after creating a = {}", Rc::strong_count(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    info!("count after creating a = {}", Rc::strong_count(&a));

    *val.borrow_mut() += 10;
    info!("muted: a: {:?}, b: {:?}, c: {:?}", a, b, c);
}
