use std::{fs::File, io::{self, ErrorKind, Read}, result};

fn read_username_from_file () -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short_hand() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let result: Result<String, io::Error> = read_username_from_file();

    println!("{:?}", result);

    let result_short_hand: Result<String, io::Error> = read_username_from_file_short_hand();
    println!("Short hand result: {:?}", result_short_hand);
}