mod input;
mod output;
mod stats;

use input::{dataset_from_file, dataset_from_rebrickable};
use std::path::Path;

use clap::{arg, Command};
/// A tool for generating lego related diagrams and visualizations.
use std::fs::read_to_string;

fn main() {
    let matches = Command::new("brickstats")
        .version("0.1")
        .author("Sebastian Seiler <sebastian.seiler@posteo.de>")
        .about("A tool for generating lego related diagrams and visualizations.")
        .arg(arg!(-s --set <VALUE>).required(false))
        .arg(arg!(-o --output <VALUE>).required(false))
        .arg(arg!(-f --file <VALUE>).required(false))
        .get_matches();

    create_directories();

    // if api token file is missing, print warning and return
    if !Path::new("secrets/api_token.txt").exists() {
        println!("Warning: api token file \"secrets/api_token.txt\" is missing");
        return;
    }
    // read api token from file "../secrets/api_token.txt"
    let api_token =
        read_to_string("secrets/api_token.txt").expect("Error reading api token from file");

    let output_file;
    let title;

    // check set and file parameters
    let dataset = match matches.get_one::<String>("set") {
        Some(set) => {
            if matches.get_one::<String>("file").is_some() {
                // print warning that file parameter is ignored
                println!("Warning: file parameter is ignored");
            }

            // unify set_num_raw to format "12345-1"
            let set_num = if set.contains('-') {
                set.to_owned()
            } else {
                format!("{}-1", set)
            };

            output_file = match matches.get_one::<String>("output") {
                // if output parameter is set to png, save as png
                Some(output) if output == "png" => Some(format!("images/{set_num}_histogram.png")),
                // if other output parameter is set, print warning
                Some(output) => {
                    println!("Warning: output parameter \"{}\" is not supported", output);
                    None
                }
                // if no output parameter is set, show dataset
                None => None,
            };

            title = format!("Parts of Set {set_num}");

            dataset_from_rebrickable(&set_num, &api_token)
        }
        None => {
            if let Some(file) = matches.get_one::<String>("file") {
                let file_path = Path::new(&file);
                if !file_path.exists() {
                    println!("Error: file \"{}\" does not exist", file);
                    return;
                }

                let file_name = file_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap();

                output_file = match matches.get_one::<String>("output") {
                    // if output parameter is set to png, save as png
                    Some(output) if output == "png" => {
                        Some(format!("images/{file_name}_histogram.png"))
                    }
                    // if other output parameter is set, print warning
                    Some(output) => {
                        println!("Warning: output parameter \"{}\" is not supported", output);
                        None
                    }
                    // if no output parameter is set, show dataset
                    None => None,
                };

                title = format!("Parts of {file_name}");

                // read dataset from file
                dataset_from_file(file, &api_token)
            } else {
                // print error that neither set nor file parameter is set
                println!("Error: neither set nor file parameter is set");
                return;
            }
        }
    };

    dataset.output(output_file, title);
}

// create "secrets", "images" and "data" directories if they don't exist
fn create_directories() {
    let directories = ["secrets", "images", "data"];
    for directory in directories.iter() {
        if !Path::new(directory).exists() {
            std::fs::create_dir(directory).unwrap();
        }
    }
}
