use regex::Regex;

/// Extracts a list of interface names from a given string of text.
/// Returns a vector of interfaces if found, otherwise an empty vector.
fn get_interfaces(text: &str) -> Vec<String> {
    let interfaces_regex = Regex::new(r"class\s+\w+\s*:\s*([\w\s,]+)").unwrap();
    let interface_name_regex = Regex::new(r"\b(\w+)\b").unwrap();

    if let Some(captures) = interfaces_regex.captures(text) {
        let interfaces_str = captures.get(1).unwrap().as_str();
        return interface_name_regex
            .find_iter(interfaces_str)
            .map(|m| m.as_str().to_string())
            .collect();
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interfaces_multiple_interfaces() {
        let text = "public class SampleClass : IFirstInterface, ISecondInterface, IDisposable";
        let result = get_interfaces(text);
        assert_eq!(
            result,
            vec![
                "IFirstInterface".to_string(),
                "ISecondInterface".to_string(),
                "IDisposable".to_string()
            ]
        );
    }

    #[test]
    fn test_get_interfaces_single_interface() {
        let text = "public class SingleInterfaceClass : ISingleInterface";
        let result = get_interfaces(text);
        assert_eq!(result, vec!["ISingleInterface".to_string()]);
    }

    #[test]
    fn test_get_interfaces_no_interfaces() {
        let text = "public class NoInterfaceClass";
        let result = get_interfaces(text);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_interfaces_empty_string() {
        let text = "";
        let result = get_interfaces(text);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_interfaces_malformed_class_definition() {
        let text = "public class MalformedClass : ";
        let result = get_interfaces(text);
        assert!(result.is_empty());
    }
}
