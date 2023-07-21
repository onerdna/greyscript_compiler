pub fn find_between<'a>(string: &'a str, first: &'a str, second: &'a str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;

    loop {
        start_index += match string[start_index..].find(first) {
            Some(value) => value + 1,
            None => break,
        };
        end_index += match string[end_index..].find(second) {
            Some(value) => value + 1,
            None => break,
        };
        result.push(string[start_index + 9..end_index - 1].to_string());
    }

    result
}
