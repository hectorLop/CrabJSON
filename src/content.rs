pub struct JSONValidator {
    clean_spaces: bool,
    validate_curly_braces: bool,
    validate_fields_format: bool,
}

impl JSONValidator {
    pub fn validate(&self, mut content: String) -> Result<(), &str> {
        if self.clean_spaces {
            content = content.replace([' ', '\n', '\t'], "");
        }

        let characters: Vec<char> = content.chars().collect();

        if self.validate_curly_braces {
            if let true = self.validate_curly_braces(&characters) {
                return Err("JSON string must start and end with curly braces");
            }
        }

        if self.validate_fields_format {
            match self.validate_format(&characters) {
                Ok(_val) => {}
                Err(err) => {
                    println!("{}", err);
                    return Err("Invalid JSON format");
                }
            };
        }

        Ok(())
    }

    fn validate_curly_braces(&self, characters: &[char]) -> bool {
        !(characters[0] == '{' && characters.last() == Some(&'}'))
    }

    fn validate_format(&self, characters: &[char]) -> Result<bool, String> {
        let mut stack: Vec<char> = vec![characters[0]];
        let mut i: usize = 1;
        let mut is_string: bool = false;

        while !stack.is_empty() {
            match characters[i] {
                '{' | '[' => stack.push(characters[i]),
                '"' => {
                    // Closer double quotes
                    if stack.last().unwrap() == &'"' {
                        stack.pop();
                        is_string = false;
                    } else {
                        // String start
                        stack.push(characters[i]);
                        is_string = true;
                    }
                }
                '}' => {
                    if !is_string {
                        if stack.last() == Some(&':') {
                            stack.pop();
                        }
                        if ![']', '"', '}'].contains(&characters[i - 1])
                            && !characters[i - 1].is_numeric()
                            && characters[i - 4..i].iter().collect::<String>() != "true"
                            && characters[i - 5..i].iter().collect::<String>() != "false"
                        {
                            return Err(format!(
                                "Invalid strUIUIUIUng => {} at position {}",
                                characters[i], i
                            ));
                        }
                        if stack.pop() != Some('{') {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                    } else if i == characters.len() - 1 {
                        return Err(format!(
                            "Invalid string => {} at position {}",
                            characters[i], i
                        ));
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return Err(format!(
                            "Invalid string => {} at position {}",
                            characters[i], i
                        ));
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    // If we found a number, it has to meet a serie of requirements
                    // if it isn't a string
                    if !is_string {
                        // If the top element in the stack is a :, then it's
                        // ok to find a number. If the top element is a [ then
                        // it's ok either. However, if they are not, then we have
                        // a problem
                        if !['[', ':'].contains(stack.last().unwrap()) {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                    }
                }
                't' | 'f' => {
                    if !is_string {
                        if i + 5 > characters.len() - 1 {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                        if !['[', ':'].contains(stack.last().unwrap()) {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                        if characters[i..i + 4].iter().collect::<String>() == "true" {
                            i += 3;
                        } else if characters[i..i + 5].iter().collect::<String>() == "false" {
                            i += 4;
                        } else {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                    }
                }
                ':' => {
                    if !is_string {
                        stack.push(characters[i]);
                    }
                }
                ',' => {
                    if !is_string && !['[', ':', '{'].contains(stack.last().unwrap()) {
                        return Err(format!(
                            "Invalid string => {} at position {}",
                            characters[i], i
                        ));
                    }
                    if stack.last() == Some(&':') {
                        stack.pop();
                    }
                }
                '.' => {
                    if !is_string {
                        // If we found a dot, and it isn't a string, then the
                        // top character on the stack bust be either [ or :
                        if !['[', ':'].contains(stack.last().unwrap()) {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                    }
                }
                '-' => {
                    // Check negative number
                    if !is_string {
                        if !['[', ':'].contains(stack.last().unwrap()) {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                        if !characters[i + 1].is_numeric() {
                            return Err(format!(
                                "Invalid string => {} at position {}",
                                characters[i], i
                            ));
                        }
                    }
                }
                _ => {
                    if !is_string {
                        return Err(format!(
                            "Invalid string => {} at position {}",
                            characters[i], i
                        ));
                    }
                }
            };
            i += 1;
        }
        Ok(true)
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
        assert_eq!(validator.validate(valid_content), Ok(()));
    }

    #[test]
    fn test_validate_fields_format() {
        let validator = JSONValidatorBuilder::new()
            .clean_spaces(true)
            .validate_fields_format(true)
            .build();

        let bad_cases = [
            "{....}",
            "{fefeff}",
            "{\"field: 2}",
            "{field: 2}",
            "{\"field\":\"2}",
            "{\"field\":fff2\"}",
            "{\"3:2}",
            "{3:2}",
            "{\"field\":\"fff2\", \"field2\": 4f}",
            "{\"field\":\"fff2\", field2: 4}",
            "{\"field\":\"fff2\", \"field2\": {\"field\": 4,},}",
            "{\"field\":\"fff2\", \"field2\": {}}",
            "{\"field\":\"fff2\", \"field2\": [3, 2, i,]}",
            "{\"field\":\"fff2\", \"field2\": [3, 2, i]}",
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
            "{\"field\":{\"aaa\":3,\"bbb\":{\"ccc\":3},\"ddd\":\"ff\"}}",
            "{\"field\":3,\"42\":3.43}",
            "{\"field\":3,\"42\":-3.43}",
            "{\"field\":true,\"42\":false}",
            "{\"field\":3,\"42\":\"3-43-43\"}",
        ];

        for case in good_cases {
            println!("Testing: {}", case);
            assert_eq!(validator.validate(case.to_string()), Ok(()));
        }
    }
}
