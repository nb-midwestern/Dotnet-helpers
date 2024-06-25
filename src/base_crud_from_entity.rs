use crate::{
    crud_query::methods::{
        generate_query_criteria, generate_sortable_field_enum, new_repo_interface_name,
        print_single_file,
    },
    utils::get_class_name::{
        extract_entity_from_base_crud_repo_class, get_class_name_and_line_number,
    },
    FieldWithType,
};

pub fn run(
    entity_name: String,
    entity_id_name: Option<String>,
    sortable_fields: Option<Vec<FieldWithType>>,
) -> String {
    let entity_id_name = entity_id_name.unwrap_or("<REPLACE_WITH_ENTITY_ID_NAME>".to_string());

    let base_project_route =
        std::env::var("BASE_PROJECT_ROUTE").unwrap_or("{No value found}".to_string());

    let base_namespace = std::env::var("BASE_NAMESPACE").ok();

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
    let new_repo = new_repository(
        entity_name.clone(),
        sortable_fields.clone(),
        base_namespace.clone(),
    );
    let new_interface = new_repo_interface_name(entity_name.clone());
    return print_single_file(
        base_project_route,
        entity_name,
        sortable_enum,
        query_criteria_class,
        new_repo,
        new_interface,
    );
}

fn new_repository(
    entity_name: String,
    sortable_fields: Option<Vec<FieldWithType>>,
    namespace: Option<String>,
) -> String {
    let namespace = match namespace {
        Some(base_namespace) => format!("namespace {base_namespace}.Infrastructure.Repositories;"),
        None => "".to_string(),
    };

    let query_override = match sortable_fields {
        Some(fields) => {
            let query:String = fields.clone().into_iter().map(|field| {
                let field_type = field.field_type.as_str();
                let field = field.field;
                match field_type {
                    x if x.contains("List<") => {
                        return format!("\n\t\t.WhereIf(criteria.{field}.Any, e => e.{field} == criteria.{field})") 
                    }
                    "string" => {
                         return format!("\n\t\t.WhereIf(!string.IsNullOrEmpty(criteria.{field}), e => e.{field} == criteria.{field})")
                    },
                    "int" => {
                        return format!("\n\t\t.WhereIf(criteria.{field}.HasValue, e => e.{field} == criteria.{field})") 
                    }
                    _ => {  return format!("\n // Check this value \n .WhereIf(criteria.{field}, e => e.{field} == criteria.{field})")},
                }
              
            }).collect();
    
format!(
r#"
    protected override IQueryable<{entity_name}> ApplyCriteria(IQueryable<{entity_name}> query, {entity_name}QueryCriteria criteria)
    {{
        //TODO check these for correctness.
        return query{query};
    }}
"#
)
        },
        None => {"".to_string()},
    };

format!(
        r#"
{namespace}

internal class {entity_name}Repository : BaseQueryCrudRepository<{entity_name}, {entity_name}QueryCriteria, {entity_name}SortableField>, I{entity_name}Repository
{{
    public {entity_name}Repository(CgwContext context) : base(context)
    {{
    }}
{query_override}
}}
    "#
    )
}
