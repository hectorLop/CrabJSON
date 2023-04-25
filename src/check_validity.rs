//pub fn is_json_valid(content: &str) -> bool {
//    let characters: Vec<char> = content.chars().collect();
//
//    if no_beginning_end_curly_braces(&characters) {
//        return false;
//    };
//
//    for (index, character) in characters.iter().enumerate() {
//        if character == &'"' {
//            if let Err(message) = field_validity(&characters, index) {
//                print!("{}", message);
//                return false;
//            }
//        }
//    }
//
//    true
//}
//
//fn no_beginning_end_curly_braces(characters: &[char]) -> bool {
//    !(characters[0] == '{' && characters.last() == Some(&'}'))
//}
//
//fn field_validity(characters: &[char], index: usize) -> Result<(), String> {
//    for (i, _element) in characters.iter().enumerate().skip(index) {
//        if !characters[i].is_alphabetic() && characters[i] != '"' {
//            return Err(format!(
//                "Invalid character: {} in position {}",
//                characters[i], i
//            ));
//        }
//    }
//
//    Ok(())
//}
//
//#[cfg(test)]
//mod test {
//    use crate::check_validity::{is_json_valid, no_beginning_end_curly_braces};
//
//    #[test]
//    fn test_invalid_start_and_end_chars() {
//        let no_curly_braces = "\"field\": 2".chars().map(|c| c).collect();
//        assert_eq!(no_beginning_end_curly_braces(&no_curly_braces), true);
//
//        let curly_brace_at_beginning = "{\"field\": 2".chars().collect();
//        assert_eq!(
//            no_beginning_end_curly_braces(&curly_brace_at_beginning),
//            true
//        );
//
//        let curly_brace_at_end = "\"field\": 2}".chars().collect();
//        assert_eq!(no_beginning_end_curly_braces(&curly_brace_at_end), true);
//
//        let valid_content = "{\"field\": 2}".chars().collect();
//        assert_eq!(no_beginning_end_curly_braces(&valid_content), false);
//    }
//
//    #[test]
//    fn test_field_validity() {
//        let no_curly_braces = "{\"field: 2}".to_string();
//        assert_eq!(is_json_valid(&no_curly_braces), false);
//    }
//}
