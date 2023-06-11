use std::{env, fs};

mod args;
mod content;
mod formatter;

use content::{JSONValidator, JSONValidatorBuilder};
use formatter::format;

fn main() {
    let filename: String = match args::validate_args(&env::args().collect()) {
        Ok(x) => x.clone(),
        Err(err) => panic!("{}", err),
    };
    println!("Filename {}", filename);

    let content = fs::read_to_string(filename).expect("Failed to read the JSON file");
    let json_validator: JSONValidator = JSONValidatorBuilder::new()
        .clean_spaces(true)
        .validate_curly_braces(true)
        .validate_fields_format(true)
        .build();

    if let Err(message) = json_validator.validate(content.clone()) {
        panic!("{}", message)
    };

    let new_content = format(content);

    fs::write("test_write.json", new_content).expect("Failed to writing the JSON file");
    println!("CrabJSON: OK");
}
