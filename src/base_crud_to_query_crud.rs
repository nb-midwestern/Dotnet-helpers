use crate::utils::{
    self,
    get_class_name::{extract_entity_from_base_crud_repo_class, get_class_name_and_line_number},
};

pub fn run(content: String) -> String {
    let entity_name =
        utils::get_class_name::extract_entity_from_base_crud_repo_class(content.clone()).unwrap();
    println!("ClassName {}", entity_name);
    let sortable_enum = generate_sortable_field_enum(entity_name.clone());
    let query_criteria_class = generate_query_criteria(entity_name.clone());
    let new_repo = new_repository_name(content.clone());
    let new_interface = new_repo_interface_name(entity_name.clone());
    return format!(
        r#"
    // SORTABLE FIELD ENUM
    // Core/Domain/{entity_name}SortableField.cs
    {sortable_enum}

    // QUERY CRITERIA CLASS
    // Core/Domain/{entity_name}QueryCriteria.cs
    {query_criteria_class}

    // Updated Repository
    // Infrastructure/Repositories/{entity_name}Repository
    {new_repo}

    //UPDATED INTERFACE
    // Infrastructure/Interfaces/I{entity_name}Repository
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

fn generate_sortable_field_enum(entity_name: String) -> String {
    return format!(
        r#"
public enum  {entity_name}SortableField
{{
    entity_Id //This needs to be updated
    EffectiveDate,
    ExpireDate, 
}}
"#
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

fn generate_query_criteria(entity_name: String) -> String {
    return format!(
        r#"
public class  {entity_name}QueryCriteria : DefaultQueryCriteria<{entity_name}SortableField>
{{
    public override SortCriteria<{entity_name}SortableField> SortCriteria {{ get; set; }} = new()
    {{
        {{ {entity_name}SortableField.entity_Id, SortOrder.Ascending }}
    }};
}}
"#
    );
}

fn new_repository_name(file_content: String) -> String {
    let entity_name = extract_entity_from_base_crud_repo_class(file_content.clone()).unwrap();
    let (class_name, class_line_number) =
        get_class_name_and_line_number(file_content.clone()).unwrap();

    let res: String = file_content
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == class_line_number {
                return format!("\n{class_name} : BaseQueryCrudRepository<{entity_name}, {entity_name}QueryCriteria, {entity_name}SortableField>, I{class_name} \n {{");
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
    ICrudRepository<{entity_name}>, IQueryCriteriaRepository<DealerRateBuildProfile, DealerRateBuildProfileQueryCriteria, DealerRateBuildProfileSortableField>
{{
        
}}
"#
    );
}
