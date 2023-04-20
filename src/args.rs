use std::path::Path;

use regex::Regex;

pub fn validate_args(args: &Vec<String>) -> Result<String, &str> {
    match apply_validation(args) {
        Ok(_) => Ok(args[1].clone()),
        Err(message) => Err(message),
    }
}

fn apply_validation(args: &Vec<String>) -> Result<bool, &str> {
    // Validate number of arguments
    match args.len() {
        1 => return Err("Must pass at least one file to format"),
        2 => (),
        _ => return Err("You shall only pass one file"),
    };

    // Validate the argument is a JSON file
    let regex = Regex::new(r"\w+\.json").unwrap();
    match regex.is_match(&args[1]) {
        true => (),
        false => return Err("The argument is not a JSON file"),
    };

    // Check the JSON file exist
    match Path::new(&args[1]).exists() {
        true => Ok(true),
        false => Err("The file doesn't exist"),
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::args::validate_args;

    #[test]
    fn test_validation() {
        let invalid_argument_number: Vec<String> =
            vec!["binary".to_owned(), "arg1".to_owned(), "arg2".to_owned()];
        assert_eq!(
            validate_args(&invalid_argument_number),
            Err("You shall only pass one file")
        );

        let argument_not_a_file: Vec<String> = vec!["binary".to_owned(), "filename.txt".to_owned()];
        let mut _file = fs::File::create(&argument_not_a_file[1]);
        assert_eq!(
            validate_args(&argument_not_a_file),
            Err("The argument is not a JSON file")
        );
        fs::remove_file(&argument_not_a_file[1]).unwrap();

        let valid_arg: Vec<String> = vec!["binary".to_owned(), "filename.json".to_owned()];
        let mut _file = fs::File::create(&valid_arg[1]);
        assert_eq!(validate_args(&valid_arg), Ok("filename.json".to_owned()));
        fs::remove_file(&valid_arg[1]).unwrap();
    }
}
