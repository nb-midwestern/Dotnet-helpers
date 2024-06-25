use crate::utils::{
    self,
    get_class_name::{extract_entity_from_base_crud_repo_class, get_class_name_and_line_number},
    get_namespace::get_base_namespace,
};

pub fn run(content: String, entity_id_name: Option<String>, sortable_fields: Option<Vec<String>>) -> String {
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

//src
// Class_name : BaseCrudRepository<Entity_name>, IEntity_name_Repository

// expected
// Class_name : BaseQueryCrudRepository<Entity_name, Entity_name_QueryCriteria, entity_nameSortableField>, IEntity_name_Repository

// CREATE enum sortable fields
// public enum entity_nameSortableField
// {
//     entity_ID
//     EffectiveDate,
//     ExpireDate,
// }

fn generate_sortable_field_enum(
    entity_name: String,
    entity_id_name: String,
    namespace: Option<String>,
    sortable_fields: Option<Vec<String>>,
) -> String {
    let namespace = match namespace {
        Some(base_namespace) => format!("namespace {base_namespace}.Core.Domain;"),
        None => "".to_string(),
    };

    let sortable_fields:String = match sortable_fields {
        Some(fields) => fields.clone().into_iter().map(|field| {return format!("{field}, \n")}).collect(),
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

// CREATE class for Entity_name_QueryCriteria
// public class Entity_name_QueryCriteria : DefaultQueryCriteria<entity_nameSortableField>
// {
//     public override SortCriteria<entity_nameSortableField> SortCriteria { get; set; } = new()
//     {
//         { entity_nameSortableField.entity_ID, SortOrder.Ascending }
//     };
// }

fn generate_query_criteria(
    entity_name: String,
    entity_id_name: String,
    namespace: Option<String>,
    sortable_fields: Option<Vec<String>>,
) -> String {

    let sortable_fields:String = match sortable_fields {
        Some(fields) => fields.clone().into_iter().map(|field| {return format!("public TYPE? {field} {{ get; set; }} \n")}).collect(),
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

fn new_repository_name(file_content: String, sortable_fields: Option<Vec<String>>) -> String {
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
                    return format!(".WhereIf(criteria.{field}.HasValue, e => e.{field} == criteria.{field})")
                }).collect();
                return format!(
r#"
    protected override IQueryable<{entity_name}> ApplyCriteria(IQueryable<{entity_name}> query, {entity_name}QueryCriteria criteria)
    {{
        //TODO check these for correctness. Strings should be string.IsNullOrEmpty(field), arrays should be field.Any
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
