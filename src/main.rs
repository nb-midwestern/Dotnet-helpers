use clap::Parser;
use dotenv::dotenv;
use itertools::Itertools;
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;
mod base_crud_to_query_crud;
mod c_sharp_dto_to_ts_interface;

mod utils;

#[derive(Clone, Debug, clap::ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Programs {
    CsDtoToTsInterface,
    GenerateQueryCriteriaFromBaseCrudClass,
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
    in_file: String,

    // Output file
    #[arg(short, long)]
    out_file: String,

    #[arg(short, long)]
    entity_id_name: Option<String>,

    #[arg(short, long)]
    base_name_space: Option<String>,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    sortable_fields: Option<Vec<String>>,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    type_sortable_fields: Option<Vec<FieldWithType>>,
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let args = Args::parse();
    println!("{:?}", args.type_sortable_fields);

    // Open the input file and read its contents
    let mut input_file = File::open(&Path::new(args.in_file.as_str()))?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;

    // Process the input content based on the program type
    let output_content: String = match args.program {
        Programs::CsDtoToTsInterface => {
            c_sharp_dto_to_ts_interface::csharp_dto_to_ts_interface(input_content)
        }
        Programs::GenerateQueryCriteriaFromBaseCrudClass => {
            base_crud_to_query_crud::run(input_content, args.entity_id_name, args.sortable_fields)
        }
    };

    // Write the processed content to the output file
    let mut output_file = File::create(&Path::new(args.out_file.as_str()))?;
    output_file.write_all(output_content.as_bytes())?;

    Ok(())
}
