pub fn interface_to_name_transform(input: &str) -> String {
    // Recursively find the innermost type name
    fn extract_innermost(input: &str) -> &str {
        if let Some(start) = input.find('<') {
            let end = input.rfind('>').unwrap_or(input.len());
            extract_innermost(&input[start + 1..end])
        } else {
            input
        }
    }

    // Get the innermost type name
    let inner_name = extract_innermost(input);

    let mut chars = inner_name.chars();
    let mut result = String::new();

    // Transform the extracted name as before
    if let Some(first_char) = chars.next() {
        if first_char == 'I' {
            if let Some(second_char) = chars.next() {
                result.push(second_char.to_ascii_lowercase());
            }
        } else {
            result.push(first_char.to_ascii_lowercase());
        }
    }

    // Append the remaining characters as-is
    result.extend(chars);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_interface_prefix() {
        assert_eq!(
            interface_to_name_transform("IUsersRepository"),
            "usersRepository"
        );
    }

    #[test]
    fn test_without_interface_prefix() {
        assert_eq!(
            interface_to_name_transform("UserRepository"),
            "userRepository"
        );
    }

    #[test]
    fn test_single_character_non_i() {
        assert_eq!(interface_to_name_transform("D"), "d");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(interface_to_name_transform(""), "");
    }

    #[test]
    fn test_multiple_uppercase_letters() {
        assert_eq!(interface_to_name_transform("IRepository"), "repository");
    }

    #[test]
    fn test_generic_interface() {
        assert_eq!(interface_to_name_transform("IOption<UserRepo>"), "userRepo");
    }

    #[test]
    fn test_nested_generic_interface() {
        assert_eq!(
            interface_to_name_transform("IResult<IOption<IUsersRepository>>"),
            "usersRepository"
        );
    }
}
