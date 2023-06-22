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

    let new_content = validate_and_format(filename);

    fs::write("test_write.json", new_content).expect("Failed to writing the JSON file");
    println!("CrabJSON: OK");
}

fn validate_and_format(filename: String) -> String {
    let content = fs::read_to_string(filename).expect("Failed to read the JSON file");
    let json_validator: JSONValidator = JSONValidatorBuilder::new()
        .clean_spaces(true)
        .validate_curly_braces(true)
        .validate_fields_format(true)
        .build();

    if let Err(message) = json_validator.validate(content.clone()) {
        panic!("{}", message)
    };

    format(content)
}

#[cfg(test)]
mod test {
    use crate::validate_and_format;
    use std::fs;

    #[test]
    fn test_validate_and_format() {
        let filenames: [&str; 3] = ["test.json", "test2.json", "test3.json"];
        let expected_filenames: [&str; 3] = [
            "expected_test.json",
            "expected_test2.json",
            "expected_test3.json",
        ];

        for i in 0..filenames.len() {
            let expected = fs::read_to_string(expected_filenames[i]).expect("Failed to read file");
            assert_eq!(expected, validate_and_format(filenames[i].to_string()));
        }
    }
}
