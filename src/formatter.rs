// Format rules:
// If they are not strings (not between ") then:
//  { -> line jump
//  : -> no whitespace behind and whitespace after
//  , -> If no in array, then line jump. If in array, then no whitespace behind
//       and whitespace after.
//  " -> After the closer ", line jump if not in array. If in array we don't do
//       nothing since we have a comma or ] after.
//
//  For this solution to be efficient, I think we can create a new array,
//  otherwise inserting in the middle will require shifting either right
//  or left, which can be expensive.

pub fn format(mut content: String) -> String {
    content = content.replace([' ', '\n', '\t'], "");
    let content_chars: Vec<char> = content.chars().collect();
    let mut path: Vec<char> = Vec::new();
    let mut is_string = false;

    let mut number_of_idents = 0;
    let mut new_content: Vec<char> = Vec::new();

    for i in 0..content_chars.len() {
        //println!("{}, {:?}", content_chars[i], new_content);
        if content_chars[i] == '{' {
            path.push(content_chars[i]);
            new_content.push(content_chars[i]);
            number_of_idents += 1;

            if content_chars[i + 1] != '\n' {
                new_content.push('\n');

                let mut tmp = vec!['\t'; number_of_idents];
                new_content.append(&mut tmp);
            }
        } else if content_chars[i] == '"' {
            new_content.push(content_chars[i]);
            if !is_string {
                is_string = true;
                path.push(content_chars[i]);
            } else if path.last() == Some(&'"') {
                is_string = false;
                path.pop();
            }
        } else if content_chars[i] == ':' {
            new_content.push(content_chars[i]);
            if !is_string {
                new_content.push(' ');
            }
        } else if content_chars[i] == ',' {
            new_content.push(content_chars[i]);
            new_content.push('\n');
            let mut tmp = vec!['\t'; number_of_idents];
            new_content.append(&mut tmp);
        } else if content_chars[i] == '}' {
            if !is_string {
                path.pop();
                number_of_idents -= 1;

                new_content.push('\n');

                let mut tmp = vec!['\t'; number_of_idents];
                new_content.append(&mut tmp);
            }
            new_content.push(content_chars[i]);
        } else if content_chars[i] == '[' {
            path.push(content_chars[i]);
            new_content.push(content_chars[i]);
            number_of_idents += 1;

            new_content.push('\n');
            let mut tmp = vec!['\t'; number_of_idents];
            new_content.append(&mut tmp);
        } else if content_chars[i] == ']' {
            if !is_string {
                path.pop();
                number_of_idents -= 1;

                new_content.push('\n');
                let mut tmp = vec!['\t'; number_of_idents];
                new_content.append(&mut tmp);
            }
            new_content.push(content_chars[i]);
        } else {
            new_content.push(content_chars[i]);
        }
    }

    new_content.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::formatter::format;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format() {
        let test_cases: Vec<(String, String)> = vec![
            (
                "{ \"key\":2, \"key\":3}".to_string(),
                "{\n\t\"key\": 2,\n\t\"key\": 3\n}".to_string(),
            ),
            (
                "{ \"key\":\"hey\", \"key\":3}".to_string(),
                "{\n\t\"key\": \"hey\",\n\t\"key\": 3\n}".to_string(),
            ),
            (
                "{ \"key\": {\"key2\": 3, \"key\": \"hey\"}, \"key\":3}".to_string(),
                "{\n\t\"key\": {\n\t\t\"key2\": 3,\n\t\t\"key\": \"hey\"\n\t},\n\t\"key\": 3\n}"
                    .to_string(),
            ),
            (
                "{ \"key\": [\"key2\", \"key\", \"hey\"], \"key\":3}".to_string(),
                "{\n\t\"key\": [\n\t\t\"key2\",\n\t\t\"key\",\n\t\t\"hey\"\n\t],\n\t\"key\": 3\n}"
                    .to_string(),
            ),
        ];
        for (input, expected) in test_cases.into_iter() {
            let result = format(input);
            assert_eq!(result, expected);
        }
    }
}
