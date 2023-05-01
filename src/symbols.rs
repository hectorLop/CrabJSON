#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    OpenBrace(char),
    CloseBrace(char),
    DoubleQuotationMarks(char),
    Letter(char),
    Colon(char),
    Number(char),
    Unspecified(String),
}

impl From<char> for Symbol {
    fn from(character: char) -> Symbol {
        match character {
            '{' => Symbol::OpenBrace(character),
            '}' => Symbol::CloseBrace(character),
            '"' => Symbol::DoubleQuotationMarks(character),
            letter if character.is_alphabetic() => Symbol::Letter(letter),
            ':' => Symbol::Colon(character),
            number if character.is_numeric() => Symbol::Number(number),
            _ => Symbol::Unspecified("Invalid symbol".to_string()),
        }
    }
}

impl Symbol {
    pub fn open_brace_actions(characters: &[char], index: usize) -> Result<(), String> {
        if index == 0 && characters[index + 1] == '"' {
            return Ok(());
        };

        return Err(format!(
            "Invalid {} at position {}",
            characters[index], index
        ));
    }

    pub fn close_brace_actions(_characters: &[char], _index: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn double_quotation_marks_action(characters: &[char], index: usize) -> Result<(), String> {
        if characters[index - 1] == '{' && characters[index + 1].is_alphanumeric() {
            return Ok(());
        }

        if characters[index - 1] == ':' && characters[index + 1].is_alphanumeric() {
            return Ok(());
        }

        if characters[index - 1].is_alphanumeric()
            && [':', ']', '}', ':'].contains(&characters[index + 1])
        {
            return Ok(());
        };

        return Err(format!(
            "Invalid {} at position {}",
            characters[index], index
        ));
    }

    pub fn letter_actions(characters: &[char], index: usize) -> Result<(), String> {
        if characters[index - 1] == '"'
            && (characters[index + 1].is_alphanumeric() || characters[index + 1] == '"')
        {
            return Ok(());
        }

        if characters[index - 1].is_alphanumeric()
            && (characters[index + 1].is_alphanumeric() || characters[index + 1] == '"')
        {
            return Ok(());
        }

        return Err(format!(
            "Invalid {} at position {}",
            characters[index], index
        ));
    }

    pub fn number_actions(_characters: &[char], _index: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn colon_actions(_characters: &[char], _index: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn unspecified_actions(_characters: &[char], _index: usize) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::symbols::Symbol;

    #[test]
    fn test_symbol_from() {
        let inputs = ['{', '}', '"', 'a', ':', '3', '='];
        let expected = [
            Symbol::OpenBrace('{'),
            Symbol::CloseBrace('}'),
            Symbol::DoubleQuotationMarks('"'),
            Symbol::Letter('a'),
            Symbol::Colon(':'),
            Symbol::Number('3'),
            Symbol::Unspecified("Invalid symbol".to_string()),
        ];

        for (input, expect) in inputs.into_iter().zip(expected.into_iter()) {
            assert_eq!(Symbol::from(input), expect)
        }
    }
}
