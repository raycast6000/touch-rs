use std::env::{args, Args};
use std::fs::File;

fn create_file(file_name: &str) -> Result<(), String> {
    match File::create_new(file_name) {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err("Couldn't write your file.".to_string())
        }
    };
}

fn main() {
    let args: Args = args();

    for (i, file_name) in args.enumerate() {
        if i == 0 {
            continue
        }

        match create_file(file_name.as_str()) {
            Ok(_) => {}
            Err(reason) => {
                println!("{}", reason.as_str());
            }
        }
    }
}