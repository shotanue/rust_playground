enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
use std::ops::Deref;


fn main() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

