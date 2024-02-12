
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

fn main(){
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {}", result);


    let both_integer: Point<i32, i32> = Point { x: 5, y: 10};
    let both_float: Point<f32, f32> = Point{x: 1.0, y: 4.0};
    let integer_and_float: Point<i32, f64> = Point {x: 5, y: 4.0};

    println!("both integer{:?} \nBoth float{:?} \nFloat and Integer{:?}", both_integer, both_float, integer_and_float);
}