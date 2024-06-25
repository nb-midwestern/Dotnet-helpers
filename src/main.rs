use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
mod base_crud_to_query_crud;
mod c_sharp_dto_to_ts_interface;
mod utils;

fn main() -> io::Result<()> {
    // Collect the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments were passed
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <program_type> <input_file> <output_file>",
            args[0]
        );
        std::process::exit(1);
    }

    // Get the program type, input, and output file paths from the arguments
    let program_type = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    // Open the input file and read its contents
    let mut input_file = File::open(&Path::new(input_path))?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;

    // Process the input content based on the program type
    let output_content = match program_type.as_str() {
        "csharp_dto_to_ts_interface" => {
            c_sharp_dto_to_ts_interface::csharp_dto_to_ts_interface(input_content)
        }
        "gen_query" => base_crud_to_query_crud::run(input_content),
        _ => {
            eprintln!("Unknown program type: {}", program_type);
            std::process::exit(1);
        }
    };

    // Write the processed content to the output file
    let mut output_file = File::create(&Path::new(output_path))?;
    output_file.write_all(output_content.as_bytes())?;

    Ok(())
}
