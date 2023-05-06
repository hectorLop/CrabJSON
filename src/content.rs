use crate::symbols::Symbol;

pub struct JSONValidator {
    clean_spaces: bool,
    validate_curly_braces: bool,
    validate_fields_format: bool,
}

impl JSONValidator {
    pub fn validate(&self, mut content: String) -> Result<String, &str> {
        if self.clean_spaces {
            content = self.clean_spaces(content);
        }

        let characters: Vec<char> = content.chars().collect();

        if self.validate_curly_braces {
            if let true = self.validate_curly_braces(&characters) {
                return Err("JSON string must start and end with curly braces");
            }
        }

        if self.validate_fields_format {
            if let false = self.validate_fields_format(&characters) {
                return Err("Invalid JSON format");
            }
        }

        Ok(content)
    }

    fn clean_spaces(&self, content: String) -> String {
        content.replace([' ', '\n', '\t'], "")
    }

    fn validate_curly_braces(&self, characters: &[char]) -> bool {
        !(characters[0] == '{' && characters.last() == Some(&'}'))
    }

    fn validate_fields_format(&self, characters: &[char]) -> bool {
        for (index, character) in characters.iter().enumerate() {
            let result = match Symbol::from(*character) {
                Symbol::OpenBrace(_c) => Symbol::open_brace_actions(characters, index),
                Symbol::CloseBrace(_c) => Symbol::close_brace_actions(characters, index),
                Symbol::DoubleQuotationMarks(_c) => {
                    Symbol::double_quotation_marks_action(characters, index)
                }
                Symbol::Letter(_c) => Symbol::letter_actions(characters, index),
                Symbol::Number(_c) => Symbol::number_actions(characters, index),
                Symbol::Colon(_c) => Symbol::colon_actions(characters, index),
                Symbol::Unspecified(_c) => Symbol::unspecified_actions(characters, index),
            };
            match result {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    return false;
                }
            };
        }

        true
    }
}

#[derive(Default)]
pub struct JSONValidatorBuilder {
    clean_spaces: bool,
    validate_curly_braces: bool,
    validate_fields_format: bool,
}

impl JSONValidatorBuilder {
    pub fn new() -> JSONValidatorBuilder {
        JSONValidatorBuilder {
            clean_spaces: false,
            validate_curly_braces: false,
            validate_fields_format: false,
        }
    }

    pub fn clean_spaces(mut self, value: bool) -> JSONValidatorBuilder {
        self.clean_spaces = value;
        self
    }

    pub fn validate_curly_braces(mut self, value: bool) -> JSONValidatorBuilder {
        self.validate_curly_braces = value;
        self
    }

    pub fn validate_fields_format(mut self, value: bool) -> JSONValidatorBuilder {
        self.validate_fields_format = value;
        self
    }

    pub fn build(self) -> JSONValidator {
        JSONValidator {
            clean_spaces: self.clean_spaces,
            validate_curly_braces: self.validate_curly_braces,
            validate_fields_format: self.validate_fields_format,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::content::JSONValidatorBuilder;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_clean_spaces() {
        let validator = JSONValidatorBuilder::new().clean_spaces(true).build();

        let input1 = "Test\ninput\twith spaces".to_string();
        assert_eq!(validator.validate(input1).unwrap(), "Testinputwithspaces")
    }

    #[test]
    fn test_validate_curly_braces() {
        let validator = JSONValidatorBuilder::new()
            .validate_curly_braces(true)
            .build();

        let no_curly_braces = "\"field\": 2".to_string();
        assert_eq!(
            validator.validate(no_curly_braces),
            Err("JSON string must start and end with curly braces")
        );

        let curly_brace_at_beginning = "{\"field\": 2".to_string();
        assert_eq!(
            validator.validate(curly_brace_at_beginning),
            Err("JSON string must start and end with curly braces")
        );

        let curly_brace_at_end = "\"field\": 2}".to_string();
        assert_eq!(
            validator.validate(curly_brace_at_end),
            Err("JSON string must start and end with curly braces")
        );

        let valid_content = "{\"field\": 2}".to_string();
        assert_eq!(
            validator.validate(valid_content),
            Ok("{\"field\": 2}".to_string())
        );
    }

    #[test]
    fn test_validate_fields_format() {
        let validator = JSONValidatorBuilder::new()
            .clean_spaces(true)
            .validate_fields_format(true)
            .build();

        let bad_cases = [
            "{\"field: 2}",
            "{field: 2}",
            "{\"field\":\"2}",
            "{\"field\":fff2\"}",
            "{\"3:2}",
            "{3:2}",
            "{\"field\":\"fff2\", \"field2\": 4f}",
            "{\"field\":\"fff2\", field2: 4}",
            "{\"field\": {\"aaa\": 3, \"bbb\": {\"ccc\": 3}, \"ddd\": \"ff\"}}",
        ];

        for case in bad_cases {
            println!("{}", case);
            assert_eq!(
                validator.validate(case.to_string()),
                Err("Invalid JSON format")
            );
        }

        let good_cases = [
            "{\"field\":\"fff2\",\"field2\":\"4f\"}",
            "{\"field\":3,\"field2\":3}",
            "{\"field\":3,\"42\":3}",
        ];

        for case in good_cases {
            println!("Testing: {}", case);
            assert_eq!(validator.validate(case.to_string()), Ok(case.to_string()));
        }
    }
}
