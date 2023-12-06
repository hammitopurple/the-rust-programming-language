use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // open_file_or_create();
    // open_file_with_unwrap();
    // open_file_with_expect();

    let username = read_username_from_file();
    match username {
        Ok(name) => println!("the username is: {name}"),
        Err(err) => println!("unable to get username from file: {:?}", err),
    }

    let username = read_username_from_file_improved();
    match username {
        Ok(name) => println!("the username is: {name}"),
        Err(err) => println!("unable to get username from file: {:?}", err),
    }

    let username = read_username_from_file_chain_operator();
    match username {
        Ok(name) => println!("the username is: {name}"),
        Err(err) => println!("unable to get username from file: {:?}", err),
    }

    let username = read_username_from_file_using_fs();
    match username {
        Ok(name) => println!("the username is: {name}"),
        Err(err) => println!("unable to get username from file: {:?}", err),
    }
}

#[allow(dead_code)]
fn open_file_or_create() -> File {
    let file = File::open("hello.txt");
    match file {
        Ok(file) => file, // return file if the file path is found
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    }
}

#[allow(dead_code)]
fn open_file_with_unwrap() -> File {
    let file = File::open("hello.txt").unwrap();
    file
}

#[allow(dead_code)]
fn open_file_with_expect() -> File {
    // The argument to "expect" should tell the user what is supposed to happen, but didn't,
    // and therefore there is a panic
    let file = File::open("hello.txt").expect("hello.txt should be included in this project");
    file
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    // Opening the file could fail
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // propagates the error upwards to the caller
    };

    let mut username = String::new();

    // Reading the file to string could fail
    // .read_to_string() appends to its argument
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // propagates the error upwards to the caller
    }
}

fn read_username_from_file_improved() -> Result<String, io::Error> {
    let mut username_file: File = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_chain_operator() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_using_fs() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
