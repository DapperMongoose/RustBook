use std::fs::File;
use std::io::{self, Read};
use std::fs;

fn main() {
    // using panic macro to cause panic
    // panic!("crash and burn");

    // cause a panic in called code by being dumb
    // let v = vec![1, 2, 3];
    //
    // v[99];

    // a messy way to handle errors opening/creating files

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    //     }
    // };

    // let unwrap help us either return the value inside the Ok result or call panic
    // let greeting_file = File::open("hello.txt").unwrap();

    // use expect to let us set our own panic message **preferred way**
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

}
// manually propogate errors upwards example
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e)
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// use the ? operator after a Result value to either return the value inside the Ok or error automatically
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// use the ? operator in a chained call
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// use the std lib instead of doing all the work ourselves
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? works when a function returns an option (some or none) result also
fn last_char_of_first_line(text : &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
