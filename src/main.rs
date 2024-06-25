use clap::Parser;
use dotenv::dotenv;
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
mod base_crud_to_query_crud;
mod c_sharp_dto_to_ts_interface;

mod utils;

#[derive(Clone, Debug, clap::ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Programs {
    CsDtoToTsInterface,
    GenerateQueryCriteriaFromBaseCrudClass,
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
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let args = Args::parse();

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
            base_crud_to_query_crud::run(input_content, args.entity_id_name)
        }
    };

    // Write the processed content to the output file
    let mut output_file = File::create(&Path::new(args.out_file.as_str()))?;
    output_file.write_all(output_content.as_bytes())?;

    Ok(())
}
