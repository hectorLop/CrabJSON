use regex::Regex;
use std::env;

pub fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    match validate_args(&args) {
        Ok(result) => args,
        Err(message) => panic!("{}", message),
    }
}

fn validate_args(args: &Vec<String>) -> Result<bool, &str> {
    match args.len() {
        1 => return Err("Must pass at least one file to format"),
        2 => (),
        _ => return Err("You shall only pass one file"),
    };

    let regex = Regex::new(r"\w+\.\w+").unwrap();

    match regex.is_match(&args[1]) {
        true => (),
        false => return Err("The argument is not a file"),
    };

    Ok(true)
}
