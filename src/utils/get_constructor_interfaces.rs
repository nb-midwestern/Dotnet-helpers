use regex::Regex;

/// Extracts a list of interfaces used in the constructor parameters of a C# class,
/// including both regular and primary constructors.
/// Returns the interfaces from the constructor with the most arguments.
pub fn get_constructor_interfaces(text: &str) -> Vec<String> {
    // Regex to match regular constructors: "public ClassName(Type param, ...)"
    let regular_constructor_regex = Regex::new(r"\b\w+\s+\w+\s*\(([^)]*)\)").unwrap();
    // Regex to match primary constructors: "public class ClassName(Type param, ...)"
    let primary_constructor_regex = Regex::new(r"class\s+\w+\s*\(([^)]*)\)").unwrap();
    // Regex to capture each interface type in the parameters, including generics
    let interface_regex = Regex::new(r"(\b\w+<[^>]+>|\b\w+)\s+\w+").unwrap();

    // Function to extract interfaces from constructor parameters
    let extract_interfaces = |captures: regex::Captures| -> (usize, Vec<String>) {
        let params = captures.get(1).unwrap().as_str();
        let interfaces: Vec<String> = interface_regex
            .captures_iter(params)
            .map(|cap| cap[1].to_string())
            .collect();
        (interfaces.len(), interfaces)
    };

    let mut max_interfaces = Vec::new();
    let mut max_arg_count = 0;

    // Extract interfaces from regular constructors
    for captures in regular_constructor_regex.captures_iter(text) {
        let (arg_count, interfaces) = extract_interfaces(captures);
        if arg_count > max_arg_count {
            max_arg_count = arg_count;
            max_interfaces = interfaces;
        }
    }

    // Extract interfaces from primary constructors
    for captures in primary_constructor_regex.captures_iter(text) {
        let (arg_count, interfaces) = extract_interfaces(captures);
        if arg_count > max_arg_count {
            max_arg_count = arg_count;
            max_interfaces = interfaces;
        }
    }

    max_interfaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_constructor_interfaces_with_generics() {
        let text = r#"
            public class TestClass
            {
                public TestClass(IComplexInterface<IFirstInterface, ISecondInterface> complexInterface, IAnotherGeneric<int> anotherGeneric) {}
                public TestClass(ISimpleInterface simpleInterface) {}
            }
        "#;
        let result = get_constructor_interfaces(text);
        assert_eq!(
            result,
            vec![
                "IComplexInterface<IFirstInterface, ISecondInterface>".to_string(),
                "IAnotherGeneric<int>".to_string()
            ]
        );
    }

    #[test]
    fn test_get_constructor_interfaces_primary_with_generics() {
        let text =
            "public class PrimaryClass(IComplexInterface<IExample, IOther> exampleInterface) {}";
        let result = get_constructor_interfaces(text);
        assert_eq!(
            result,
            vec!["IComplexInterface<IExample, IOther>".to_string()]
        );
    }

    #[test]
    fn test_get_constructor_interfaces_single_generic_interface() {
        let text = "public class SingleGenericClass(ISingleGeneric<IGeneric> generic) {}";
        let result = get_constructor_interfaces(text);
        assert_eq!(result, vec!["ISingleGeneric<IGeneric>".to_string()]);
    }

    #[test]
    fn test_get_constructor_interfaces_no_interfaces() {
        let text = "public class NoInterfacesClass {}";
        let result = get_constructor_interfaces(text);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_constructor_interfaces_empty_string() {
        let text = "";
        let result = get_constructor_interfaces(text);
        assert!(result.is_empty());
    }
}
