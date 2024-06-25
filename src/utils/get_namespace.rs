use regex::Regex;

pub fn get_base_namespace(file_data: String) -> Option<String> {
    let re = Regex::new(r"^namespace\s+([a-zA-Z0-9_]+)").unwrap();

    for line in file_data.lines() {
        if let Some(captures) = re.captures(&line) {
            if let Some(namespace) = captures.get(1) {
                return Some(namespace.as_str().to_string());
            }
        }
    }
    None
}
