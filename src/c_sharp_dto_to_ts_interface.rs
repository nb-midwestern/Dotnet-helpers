pub fn csharp_dto_to_ts_interface(content: String) -> String {
    // Parse the C# DTO and generate the TypeScript interface
    let ts_interface = convert_to_typescript_interface(&content);
    ts_interface
}

fn convert_to_typescript_interface(dto_content: &str) -> String {
    let binding = dto_content
        .trim()
        .replace('\n', "")
        .replace('\t', "")
        .replace('\r', "");
    let binding: Vec<&str> = binding.split(' ').collect();

    let mut arrays: Vec<Vec<&str>> = Vec::new();
    let mut current_array: Vec<&str> = Vec::new();
    for line in binding.clone() {
        if line.trim().is_empty() {
            if !current_array.is_empty() {
                arrays.push(current_array.clone());
                current_array.clear();
            }
        } else {
            current_array.push(line);
        }
    }

    let mut ts_interface = String::new();

    for array in arrays {
        // PROCESS each line
        if array.contains(&"class") {
            let mut previous_was_class = false;
            for item in array {
                if previous_was_class {
                    previous_was_class = false;
                    let item = item.split("}").collect::<Vec<_>>()[0];
                    ts_interface.push_str(item);
                    ts_interface.push_str("{\n")
                }
                if item == "class" {
                    previous_was_class = true;
                    ts_interface.push_str("interface ")
                }
            }
        } else {
            let mut previous_was_access_modifier = false;
            let mut var_type = " ";
            let mut is_optional = false;
            for item in array {
                if var_type != " " {
                    let mut item = item.to_string();
                    let item = item.remove(0).to_lowercase().to_string() + &item;
                    ts_interface.push_str(&item);
                    if is_optional {
                        ts_interface.push('?');
                    }
                    ts_interface.push_str(": ");
                    ts_interface.push_str(var_type);
                    ts_interface.push_str(";\n");
                    var_type = " ";
                }
                if previous_was_access_modifier {
                    previous_was_access_modifier = false;
                    let split_type = item.split('?').collect::<Vec<&str>>();
                    if item.contains(&"?") {
                        is_optional = true;
                    }
                    var_type = convert_type(split_type[0]);
                }
                if item == "public" || item == "private" {
                    previous_was_access_modifier = true;
                }
            }
        }
        // println!("{:?}", array);
    }

    ts_interface.push_str("}\n");

    // ts_interface
    ts_interface.to_string()
}

fn convert_type(csharp_type: &str) -> &str {
    match csharp_type {
        "int" => "number",
        "float" => "number",
        "double" => "number",
        "decimal" => "number",
        "string" => "string",
        "bool" => "boolean",
        "DateTime" => "Date",
        "List<int>" => "number[]",
        "List<string>" => "string[]",
        _ => "any", // Default to `any` for unknown types
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn remove_whitespace(input: &str) -> String {
        input.chars().filter(|&c| !c.is_whitespace()).collect()
    }
    #[test]
    fn test_convert_simple_dto() {
        let csharp_dto = r#"
            public class SimpleDto 
            {
                public int Id { get; set; }
                public string? Name { get; set; }
                public bool IsActive { get; set; }
            }
        "#;

        let expected_ts_interface = r#"
            interface SimpleDto {
                id: number;
                name?: string;
                isActive: boolean;
            }
            "#;

        let ts_interface = convert_to_typescript_interface(csharp_dto);
        assert_eq!(
            remove_whitespace(ts_interface.as_str()),
            remove_whitespace(expected_ts_interface)
        );
    }

    #[test]
    fn test_convert_complex_dto() {
        let csharp_dto = r#"
                public class ComplexDto {
                    public int Id { get; set; }
                    public string Name { get; set; }
                    public bool IsActive { get; set; }
                    public DateTime CreatedAt { get; set; }
                    public List<string> Tags { get; set; }
                }
            "#;

        let expected_ts_interface = r#"
                interface ComplexDto {
                    id: number;
                    name: string;
                    isActive: boolean;
                    createdAt: Date;
                    tags: string[];
                }
                "#;

        let ts_interface = convert_to_typescript_interface(csharp_dto);

        assert_eq!(
            remove_whitespace(ts_interface.as_str()),
            remove_whitespace(expected_ts_interface)
        );
    }

    #[test]
    fn test_unknown_type() {
        let csharp_dto = r#"
                public class UnknownTypeDto {
                    public CustomType CustomProperty { get; set; }
                }
            "#;

        let expected_ts_interface = r#"
                interface UnknownTypeDto {
                    customProperty: any;
                }
                "#;

        let ts_interface = convert_to_typescript_interface(csharp_dto);
        assert_eq!(
            remove_whitespace(ts_interface.as_str()),
            remove_whitespace(expected_ts_interface)
        );
    }

    #[test]
    fn test_empty_class() {
        let csharp_dto = r#"
                public class EmptyDto {
                }
            "#;

        let expected_ts_interface = r#"
            interface EmptyDto {
            }
            "#;

        let ts_interface = convert_to_typescript_interface(csharp_dto);

        assert_eq!(
            remove_whitespace(ts_interface.as_str()),
            remove_whitespace(expected_ts_interface)
        );
    }
}
