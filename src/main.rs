use std::{env, fs};

mod args;
//mod check_validity;
mod content;

use content::{JSONValidator, JSONValidatorBuilder};

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

    let _clean_content = match json_validator.validate(content) {
        Ok(result) => result,
        Err(message) => panic!("{}", message),
    };
    //let clean_content = content.replace([' ', '\n', '\t'], "");

    //match check_validity::is_json_valid(&clean_content) {
    //    true => (),
    //    false => panic!("Invalid JSON"),
    //}
}
