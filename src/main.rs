// Smart pointers

fn smart_pointers() -> () {
    let amount = Box::new(8700);
    println!("The amount contributed is {}", amount);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main(){
    println!("Hello World");
    smart_pointers();
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn ref_test(){
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}

#[test]
fn deref_test_impl(){
    let x = 5;
    let y: MyBox<i32> = MyBox::new(x);
 
    assert_eq!(x, 5);
    assert_eq!(5, *y);
}