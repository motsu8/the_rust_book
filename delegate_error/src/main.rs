use std::fs::File;
use std::io::{self, Read};

fn main() {
    read_username_from_file();
    refactor_read_username_from_file();
    refactor_2_read_username_from_file();
    refactor_3_read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn refactor_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_sting(&mut username)?;
    Ok(username)
}

fn refactor_2_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_sting(&mut username)?;
    Ok(username)
}

fn refactor_3_read_username_from_file() -> Result<String, io::Error> {
    use::std::fs;
    fs::read_to_sting("hello.txt")
}
