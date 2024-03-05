// Smart pointers
use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

fn smart_pointers() -> () {
    let amount = MyBox::new(8700);
    println!("The amount contributed is {:?}", amount);
}

fn custom_smart_pointers() {
    // let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let c = CustomSmartPointer {
        data: String::from("my Value"),
    };

    let d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("My other value"),
    };

    assert_eq!(c.data, "my Value");
    assert_eq!(d.data, "My other value");

    println!("Custom Smart Pointers Created!!!");
}

fn rc_pointer_counted() {
    let k: Rc<List<i32>> = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("Count after creating k = {}", Rc::strong_count(&k));
    let _l: List<i32> = Cons(2, Rc::clone(&k));
    println!("Count after creating l = {}", Rc::strong_count(&k));
    {
        let _m: List<i32> = Cons(4, Rc::clone(&k));
        println!("Count after creating m = {}", Rc::strong_count(&k));
    }

    println!("Count after m goes out of scope = {}", Rc::strong_count(&k))
}



fn main() {
    println!("Hello World");
    smart_pointers();
    rc_pointer_counted();
    custom_smart_pointers();

    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));

    let a: Rc<List<Rc<RefCell<i32>>>> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b: List<Rc<RefCell<i32>>> = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c: List<Rc<RefCell<i32>>> = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom SmartPointer with data `{}`", self.data);
    }
}

#[cfg(test)]
#[test]
fn ref_test() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}

#[test]
fn deref_test_impl() {
    let x = 5;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(5, *y);
}

#[test]
fn tests_custom_smart_pointers() {
    let c = CustomSmartPointer {
        data: String::from("my Value"),
    };

    let d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("My other value"),
    };

    assert_eq!(c.data, "my Value");
    assert_eq!(d.data, "My other value");
}
