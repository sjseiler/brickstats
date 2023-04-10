mod input;
mod output;
mod stats;

use input::{formatted_inventory, inventory_from_file, prepare_dataset, Rebrickable};
use output::wordcloud;
use std::fs;
use std::path::Path;

use clap::{self, arg};
/// A tool for generating lego related diagrams and visualizations.
use std::fs::read_to_string;

fn main() {
    let matches = clap::Command::new("brickstats")
        .version("0.1")
        .author("Sebastian Seiler <sebastian.seiler@posteo.de>")
        .about("A tool for generating lego related diagrams and visualizations.")
        .arg(arg!(-s --set <VALUE>).required(false))
        .arg(arg!(-o --output <VALUE>).required(false))
        .arg(arg!(-f --file <VALUE>).required(false))
        .arg(arg!(-w - -wordcloud).required(false))
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
    let rebrickable = Rebrickable::new(api_token);

    let output_file_prefix;
    let title;

    // check set and file parameters
    let inventory = match matches.get_one::<String>("set") {
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

            output_file_prefix = match matches.get_one::<String>("output") {
                // if output parameter is set to png, save as png
                Some(output) if output == "png" => Some(format!("images/{set_num}")),
                // if other output parameter is set, print warning
                Some(output) => {
                    println!("Warning: output parameter \"{}\" is not supported", output);
                    None
                }
                // if no output parameter is set, show dataset
                None => None,
            };

            title = format!("Parts of Set {set_num}");

            // download set inventory
            rebrickable.inventory(&set_num)
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

                output_file_prefix = match matches.get_one::<String>("output") {
                    // if output parameter is set to png, save as png
                    Some(output) if output == "png" => Some(format!("images/{file_name}")),
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
                inventory_from_file(file)
            } else {
                // print error that neither set nor file parameter is set
                println!("Error: neither set nor file parameter is set");
                return;
            }
        }
    };

    // fetch part, category and color details from rebrickable
    let colors = rebrickable.all_colors();
    let categories = rebrickable.all_categories();
    let part_details = rebrickable.part_details(&inventory);

    // if wordcloud parameter is set, create wordcloud
    if matches.get_flag("wordcloud") {
        // create formatted inventory
        let formatted_inventory = formatted_inventory(&inventory, &part_details, &colors);

        // create filename
        // either use set_num or input file name
        let file_name = format!("{}_wordcloud.png", output_file_prefix.as_ref().unwrap());

        wordcloud(formatted_inventory, &file_name).expect("failed to write wordcloud text file");
    }

    // prepare data for plot
    let dataset = prepare_dataset(inventory, part_details, categories, colors);
    dataset.output(output_file_prefix, title);

    // remove temp files
    fs::remove_dir_all("temp/").expect("failed to remove temp files");
}

// create "secrets", "images" and "data" directories if they don't exist
fn create_directories() {
    let directories = ["secrets", "images", "data", "temp"];
    for directory in directories.iter() {
        if !Path::new(directory).exists() {
            std::fs::create_dir(directory).unwrap();
        }
    }
}
