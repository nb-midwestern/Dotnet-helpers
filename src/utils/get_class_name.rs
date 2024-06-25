use regex::Regex;

pub fn get_class_name(file_data: String) -> Option<String> {
    let class_regex = Regex::new(r"(?m)^\s*public\s+class\s+(\w+)").unwrap();

    if let Some(captures) = class_regex.captures(&file_data) {
        if let Some(class_name) = captures.get(1) {
            return Some(class_name.as_str().to_string());
        }
    }

    None
}

pub fn get_class_name_and_line_number(file_data: String) -> Option<(String, usize)> {
    // Define the regex pattern
    let class_regex = Regex::new(r"^\s*public\s+class\s+(\w+)").unwrap();

    // Iterate through each line with line numbers
    for (index, line) in file_data.lines().enumerate() {
        if let Some(captures) = class_regex.captures(&line) {
            if let Some(class_name) = captures.get(1) {
                return Some((class_name.as_str().to_string(), index + 1));
            }
        }
    }

    None
}

pub fn extract_entity_from_base_crud_repo_class(file_data: String) -> Option<String> {
    let re =
        Regex::new(r"(?m)^\s*public\s+class\s+\w+\s*:\s*\w+(?:<(\w+)>)?\s*(?:,|\{|$)").unwrap();
    if let Some(captures) = re.captures(file_data.as_str()) {
        if let Some(generic_type) = captures.get(1) {
            return Some(generic_type.as_str().to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_generic_type() {
        let declaration =
            "public class CustomerRepository : BaseCrudRepository<Customer>, ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            Some("Customer".to_string())
        );
    }

    #[test]
    fn test_no_generic_type() {
        let declaration = "public class CustomerRepository : ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            None
        );
    }

    #[test]
    fn test_multiple_generic_types() {
        let declaration = "public class CustomerRepository : BaseCrudRepository<Customer, Order>, ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            Some("Customer".to_string())
        );
        // It will only capture the first generic type
    }

    #[test]
    fn test_generic_type_with_numbers() {
        let declaration = "public class CustomerRepository : BaseCrudRepository<Customer123>, ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            Some("Customer123".to_string())
        );
    }

    #[test]
    fn test_generic_type_with_underscores() {
        let declaration = "public class CustomerRepository : BaseCrudRepository<Customer_Type>, ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            Some("Customer_Type".to_string())
        );
    }

    #[test]
    fn test_empty_generic_type() {
        let declaration =
            "public class CustomerRepository : BaseCrudRepository<>, ICustomerRepository";
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            None
        );
    }

    #[test]
    fn test_multiline_class() {
        let declaration = r#"
            public class OrderRepository :
                BaseCrudRepository<Order>,
                IOrderRepository {
            "#;
        assert_eq!(
            extract_entity_from_base_crud_repo_class(declaration.to_string()),
            Some("Order".to_string())
        );
    }
}
