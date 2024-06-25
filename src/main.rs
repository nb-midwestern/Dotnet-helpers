use clap::Parser;
use dotenv::dotenv;
use itertools::Itertools;
use serde::Serialize;
use std::fs::File;
use std::io::{self, Error, Read, Write};
use std::os::unix::process;
use std::path::Path;
use std::process::Output;
use std::str::FromStr;
mod base_crud_from_entity;
mod base_crud_to_query_crud;
mod c_sharp_dto_to_ts_interface;
mod crud_query;

mod utils;

#[derive(Clone, Debug, clap::ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Programs {
    CsDtoToTsInterface,
    GenerateQueryCriteriaFromBaseCrudClass,
    GenerateQueryCriterialFromEntityName,
}

#[derive(Clone, Debug)]
struct FieldWithType {
    field: String,
    field_type: String,
}

impl FromStr for FieldWithType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, field_type) = s
            .split(",")
            .collect::<Vec<_>>()
            .into_iter()
            .collect_tuple()
            .unwrap();

        Ok(Self {
            field: field.to_string(),
            field_type: field_type.to_string(),
        })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What program should be ran
    #[arg(short, long)]
    program: Programs,

    /// Input file
    #[arg(short, long)]
    in_file: Option<String>,

    /// Output file
    #[arg(short, long)]
    out_file: String,

    /// name of the primary key
    #[arg(short, long)]
    entity_id_name: Option<String>,

    #[arg(short, long)]
    base_name_space: Option<String>,

    /// Sort field and its type, comma separated. IE: name,string id,int
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    type_sortable_fields: Option<Vec<FieldWithType>>,

    /// name of the entity
    #[arg(short = 'x', long)]
    entity_name: Option<String>,
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let args = Args::parse();
    println!("{:?}", args.type_sortable_fields);

    // Process the input content based on the program type
    let output_content: String = match args.program {
        Programs::CsDtoToTsInterface => c_sharp_dto_to_ts_interface::csharp_dto_to_ts_interface(
            read_from_file(args.in_file.expect("Input file must be specified").as_str())
                .expect("Error reading file"),
        ),
        Programs::GenerateQueryCriteriaFromBaseCrudClass => base_crud_to_query_crud::run(
            read_from_file(args.in_file.expect("Input file must be specified").as_str())
                .expect("Error reading file"),
            args.entity_id_name,
            args.type_sortable_fields,
        ),
        Programs::GenerateQueryCriterialFromEntityName => base_crud_from_entity::run(
            args.entity_name.expect("Entity Name field is required"),
            args.entity_id_name,
            args.type_sortable_fields,
        ),
    };

    // Write the processed content to the output file
    let out_file_name = args.out_file;
    let mut output_file = File::create(&Path::new(out_file_name.clone().as_str()))?;
    output_file.write_all(output_content.as_bytes())?;

    println!(
        r#"
    ███████ ██ ██      ███████      ██████  ███████ ███    ██ ███████ ██████   █████  ████████ ███████ ██████  
    ██      ██ ██      ██          ██       ██      ████   ██ ██      ██   ██ ██   ██    ██    ██      ██   ██ 
    █████   ██ ██      █████       ██   ███ █████   ██ ██  ██ █████   ██████  ███████    ██    █████   ██   ██ 
    ██      ██ ██      ██          ██    ██ ██      ██  ██ ██ ██      ██   ██ ██   ██    ██    ██      ██   ██ 
    ██      ██ ███████ ███████      ██████  ███████ ██   ████ ███████ ██   ██ ██   ██    ██    ███████ ██████  
       located at {out_file_name}                                                                                        
    "#
    );

    Ok(())
}

fn read_from_file(input_file: &str) -> Result<String, Error> {
    // Open the input file and read its contents
    let mut input_file = File::open(&Path::new(input_file))?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;
    Ok(input_content)
}
