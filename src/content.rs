pub struct JSONValidator {
    clean_spaces: bool,
    validate_curly_braces: bool,
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

        Ok(content)
    }

    fn clean_spaces(&self, content: String) -> String {
        content.replace([' ', '\n', '\t'], "")
    }

    fn validate_curly_braces(&self, characters: &[char]) -> bool {
        !(characters[0] == '{' && characters.last() == Some(&'}'))
    }
}

#[derive(Default)]
pub struct JSONValidatorBuilder {
    clean_spaces: bool,
    validate_curly_braces: bool,
}

impl JSONValidatorBuilder {
    pub fn new() -> JSONValidatorBuilder {
        JSONValidatorBuilder {
            clean_spaces: false,
            validate_curly_braces: false,
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

    pub fn build(self) -> JSONValidator {
        JSONValidator {
            clean_spaces: self.clean_spaces,
            validate_curly_braces: self.validate_curly_braces,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::content::JSONValidatorBuilder;

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
}
