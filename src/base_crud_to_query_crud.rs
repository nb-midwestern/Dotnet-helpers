use crate::{crud_query::methods::{generate_query_criteria, generate_sortable_field_enum, print_single_file}, utils::{
    self,
    get_class_name::{extract_entity_from_base_crud_repo_class, get_class_name_and_line_number},
    get_namespace::get_base_namespace,
}, FieldWithType};

pub fn run(content: String, entity_id_name: Option<String>, sortable_fields: Option<Vec<FieldWithType>>) -> String {
    let entity_id_name = entity_id_name.unwrap_or("<REPLACE_WITH_ENTITY_ID_NAME>".to_string());

    let base_project_route =
        std::env::var("BASE_PROJECT_ROUTE").unwrap_or("{No value found}".to_string());

    let base_namespace = get_base_namespace(content.clone());

    let entity_name =
        utils::get_class_name::extract_entity_from_base_crud_repo_class(content.clone()).unwrap();
    println!("ClassName {}", entity_name);
    let sortable_enum = generate_sortable_field_enum(
        entity_name.clone(),
        entity_id_name.clone(),
        base_namespace.clone(),
        sortable_fields.clone(),
    );
    let query_criteria_class = generate_query_criteria(
        entity_name.clone(),
        entity_id_name.clone(),
        base_namespace.clone(),
        sortable_fields.clone(),
    );
    let new_repo = new_repository_name(content.clone(),         sortable_fields.clone(),);
    let new_interface = new_repo_interface_name(entity_name.clone());
    return print_single_file(base_project_route, entity_name, sortable_enum, query_criteria_class, new_repo, new_interface) 
}


fn new_repository_name(file_content: String, sortable_fields: Option<Vec<FieldWithType>>) -> String {
    let has_sortable_fields = match sortable_fields {
        Some(_) => {true},
        None => {false},
    };
    let entity_name = extract_entity_from_base_crud_repo_class(file_content.clone()).unwrap();
    let (class_name, class_line_number) =
        get_class_name_and_line_number(file_content.clone()).unwrap();


    let res: String = file_content
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == class_line_number  {
                return format!("\n internal class {class_name} : BaseQueryCrudRepository<{entity_name}, {entity_name}QueryCriteria, {entity_name}SortableField>, I{class_name}");
            } else if has_sortable_fields && line.trim() == "}" && file_content.lines().skip(index + 1).all(|l| l.trim().is_empty()) {
                let query:String = sortable_fields.clone().unwrap().into_iter().map(|field| {
                    let field_type = field.field_type.as_str();
                    let field = field.field;
                    match field_type {
                        x if x.contains("[]") => {
                            return format!("\n.WhereIf(criteria.{field}.Any, e => e.{field} == criteria.{field})") 
                        }
                        "string" => {
                             return format!("\n.WhereIf(!string.IsNullOrEmpty({field}), e => e.{field} == criteria.{field})")
                        },
                        "int" => {
                            return format!("\n.WhereIf(criteria.{field}.HasValue, e => e.{field} == criteria.{field})") 
                        }
                        _ => {  return format!("\n // Check this value \n .WhereIf(criteria.{field}, e => e.{field} == criteria.{field})")},
                    }
                  
                }).collect();
                return format!(
r#"
    protected override IQueryable<{entity_name}> ApplyCriteria(IQueryable<{entity_name}> query, {entity_name}QueryCriteria criteria)
    {{
        //TODO check these for correctness.
        return query{query};
    }}
}}
"#
)
            } 
            
            format!("\n{line}")
            
        })
        .collect();

    return format!("{}", res);
}

fn new_repo_interface_name(entity_name: String) -> String {
    return format!(
        r#"
public interface I{entity_name}Repository :
    ICrudRepository<{entity_name}>, IQueryCriteriaRepository<{entity_name}, {entity_name}QueryCriteria, {entity_name}SortableField>
{{
        
}}
"#
    );
}
