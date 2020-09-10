use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_username_from_file();

    match result {
        Ok(username) => println!("Got username: {}", username),
        Err(error) => panic!("Failed to read username: {:?}", error),
    }
}
