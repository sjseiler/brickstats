mod input;
mod output;
mod stats;

use input::get_dataset_from_rebrickable;
use output::open_histogram_in_gnuplot;

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

    // print warning for dummy parameter "output"
    if let Some(_output) = matches.get_one::<String>("output") {
        println!("Warning: output parameter is not yet implemented");
    }

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
    open_histogram_in_gnuplot(dataset);
}
