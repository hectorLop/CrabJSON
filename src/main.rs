use std::{env, fs};

mod args;
mod check_validity;

fn main() {
    let filename: String = match args::validate_args(&env::args().collect()) {
        Ok(x) => x.clone(),
        Err(err) => panic!("{}", err),
    };
    println!("Filename {}", filename);

    let content = fs::read_to_string(filename).expect("Failed to read the JSON file");
    let clean_content = content.replace([' ', '\n', '\t'], "");

    match check_validity::is_json_valid(&clean_content) {
        true => (),
        false => panic!("Invalid JSON"),
    }
}
