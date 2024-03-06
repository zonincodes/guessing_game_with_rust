use core::slice;

use learn::Post;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        print_hello();
    }

    let mut vector = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = split_at_mut(&mut vector, 3);

    println!("Left = {:?} \nRight = {:?}", left, right);

}

fn split_at_mut(values: &mut[i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.add(mid), len -mid))
    }
}

unsafe fn print_hello() {
    println!("Hello World!");
}


fn _oop_impl() {
    println!("Hello World!");
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
