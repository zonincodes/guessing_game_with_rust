use std::io;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn vectors() {
    let numbers = vec![5, 3, 5, 1, 3, 20];

    let max_element = numbers.iter().max();

    match max_element {
        Some(&max) => println!("Maximum element: {}", max),
        None => println!("The vector is empty."),
    }
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index 1-5: ");
  
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read");

    let index: usize = index.trim().parse().expect("Not a  number");

    let element: i32 = a[index -1];

    println!("The value of the elemnt at index {} is: {element}", index -1);
}
fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {}", result);

    let both_integer: Point<i32, i32> = Point { x: 5, y: 10 };
    let both_float: Point<f32, f32> = Point { x: 1.0, y: 4.0 };
    let integer_and_float: Point<i32, f64> = Point { x: 5, y: 4.0 };

    println!(
        "both integer{:?} \nBoth float{:?} \nFloat and Integer{:?}",
        both_integer, both_float, integer_and_float
    );
    vectors();
    arrays();

    let mut s = String::from("Hello");
    change(&mut s);

    println!("{s}!");
}
