// CREATE enum sortable fields
// public enum entity_nameSortableField
// {
//     entity_ID
//     EffectiveDate,
//     ExpireDate,
// }

use crate::FieldWithType;

pub fn generate_sortable_field_enum(
    entity_name: String,
    entity_id_name: String,
    namespace: Option<String>,
    sortable_fields: Option<Vec<FieldWithType>>,
) -> String {
    let namespace = match namespace {
        Some(base_namespace) => format!("namespace {base_namespace}.Core.Domain;"),
        None => "".to_string(),
    };

    let sortable_fields: String = match sortable_fields {
        Some(fields) => fields
            .clone()
            .into_iter()
            .map(|field| return format!("{}, \n", field.field))
            .collect(),
        None => "".to_string(),
    };
    return format!(
        r#"
{namespace}

public enum {entity_name}SortableField
{{
    {entity_id_name},
    EffectiveDate,
    ExpireDate, 
    {sortable_fields}
}}
"#,
    );
}

pub fn generate_query_criteria(
    entity_name: String,
    entity_id_name: String,
    namespace: Option<String>,
    sortable_fields: Option<Vec<FieldWithType>>,
) -> String {
    let sortable_fields: String = match sortable_fields {
        Some(fields) => fields
            .clone()
            .into_iter()
            .map(|field| {
                return format!(
                    "public {}? {} {{ get; set; }} \n",
                    field.field_type, field.field
                );
            })
            .collect(),
        None => "".to_string(),
    };

    let namespace = match namespace {
        Some(base_namespace) => format!("namespace {base_namespace}.Core.Domain;"),
        None => "".to_string(),
    };

    return format!(
        r#"
{namespace}

public class  {entity_name}QueryCriteria : DefaultQueryCriteria<{entity_name}SortableField>
{{
    {sortable_fields}
    public override SortCriteria<{entity_name}SortableField> SortCriteria {{ get; set; }} = new()
    {{
        {{ {entity_name}SortableField.{entity_id_name}, SortOrder.Ascending }}
    }};
}}
"#
    );
}

pub fn new_repo_interface_name(entity_name: String) -> String {
    return format!(
        r#"
public interface I{entity_name}Repository :
    ICrudRepository<{entity_name}>, IQueryCriteriaRepository<{entity_name}, {entity_name}QueryCriteria, {entity_name}SortableField>
{{
        
}}
"#
    );
}

pub fn print_single_file(
    base_project_route: String,
    entity_name: String,
    sortable_enum: String,
    query_criteria_class: String,
    new_repo: String,
    new_interface: String,
) -> String {
    return format!(
        r#"
    // SORTABLE FIELD ENUM
    // touch {base_project_route}Core/Domain/{entity_name}SortableField.cs
{sortable_enum}

    // QUERY CRITERIA CLASS
    // touch {base_project_route}Core/Domain/{entity_name}QueryCriteria.cs
{query_criteria_class}

    // Updated Repository
    // touch {base_project_route}Infrastructure/Repositories/{entity_name}Repository
{new_repo}

    //UPDATED INTERFACE
    // touch {base_project_route}Infrastructure/Interfaces/I{entity_name}Repository
{new_interface}
    "#,
    );
}
