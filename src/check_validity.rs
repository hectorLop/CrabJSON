pub fn is_json_valid(content: &str) -> bool {
    let characters: Vec<char> = content.chars().collect();

    if !(characters[0] == '{' && characters.last() == Some(&'}')) {
        return false;
    };

    //for (index, character) in characters.iter().enumerate() {
    //    if character == &'"' {
    //        if let Err(message) = field_validity(&characters, index) {
    //            print!("{}", message);
    //            return false;
    //        }
    //    }
    //}

    true
}

//fn field_validity(characters: &Vec<char>, index: usize) -> Result<(), String> {
//    for i in index..characters.len() {
//        if !characters[i].is_alphabetic() && characters[i] != '"'{
//            return Err(format!("Invalid character: {} in position {}", characters[i], i));
//        }
//    }
//
//    Ok(())
//}

#[cfg(test)]
mod test {
    use crate::check_validity::is_json_valid;

    #[test]
    fn test_invalid_start_and_end_chars() {
        let no_curly_braces = "\"field\": 2".to_string();
        assert_eq!(is_json_valid(&no_curly_braces), false);

        let curly_brace_at_beginning = "{\"field\": 2".to_string();
        assert_eq!(is_json_valid(&curly_brace_at_beginning), false);

        let curly_brace_at_end = "\"field\": 2}".to_string();
        assert_eq!(is_json_valid(&curly_brace_at_end), false);

        let valid_content = "{\"field\": 2}".to_string();
        assert_eq!(is_json_valid(&valid_content), true);
    }

    //#[test]
    //fn test_field_validity() {
    //    let no_curly_braces = "{\"field: 2}".to_string();
    //    assert_eq!(is_json_valid(&no_curly_braces), false);

    //}
}
