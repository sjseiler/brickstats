mod input;
mod output;
mod stats;

use input::get_dataset_from_rebrickable;

use clap::{arg, Command};
/// A tool for generating lego related diagrams and visualizations.
use std::fs::read_to_string;

fn main() {
    let matches = Command::new("brickstats")
        .version("0.1")
        .author("Sebastian Seiler <sebastian.seiler@posteo.de>")
        .about("A tool for generating lego related diagrams and visualizations.")
        .arg(arg!(-s --set <VALUE>).required(true))
        .arg(arg!(-o --output <VALUE>).required(false))
        .get_matches();

    // get set num from command line arguments in either format "12345-1" or "12345"
    let set_num_raw = matches.get_one::<String>("set").expect("required");

    // unify set_num_raw to format "12345-1"
    let set_num = if set_num_raw.contains('-') {
        set_num_raw.to_owned()
    } else {
        format!("{}-1", set_num_raw)
    };

    // read api token from file "../secrets/api_token.txt"
    let api_token =
        read_to_string("secrets/api_token.txt").expect("Couldn't read api token from file");

    let dataset = get_dataset_from_rebrickable(&set_num, &api_token);

    // show or save dataset based on output parameter
    let output = match matches.get_one::<String>("output") {
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
    dataset.output(output);
}
