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
