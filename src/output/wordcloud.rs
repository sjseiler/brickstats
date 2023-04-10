use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::Command;

const TEMP_TEXT_FILE: &str = "temp/wordcloud.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryEntry {
    color: String,
    part_name: String,
    quantity: i32,
}

impl InventoryEntry {
    pub fn new(color: String, part_name: String, quantity: i32) -> InventoryEntry {
        InventoryEntry {
            color,
            part_name,
            quantity,
        }
    }
}

// create text file as input for wordcloud from
fn text_file(inventory_entries: Vec<InventoryEntry>) -> Result<()> {
    // open file
    let mut file = File::create(TEMP_TEXT_FILE)?;

    for entry in inventory_entries {
        // remove list of terms from color name
        // * " Reddish "
        // * " Bluish "
        let mut color_formatted = entry.color.replace("Reddish ", "");
        color_formatted = color_formatted.replace(" Bluish ", " ");
        color_formatted = color_formatted.replace(' ', "_");

        // find _ x _ x _ and replace with _x_x_ in part name
        let mut part_name_formatted = entry.part_name.replace(" x ", "x");

        // replace written numbers with numbers ignoring case using regex
        let re = Regex::new(r"(?i)\s(one|two|three|four|five|six|seven|eight|nine|ten)\s").unwrap();
        part_name_formatted = re
            .replace_all(&part_name_formatted, |caps: &regex::Captures| {
                let number = caps.get(1).unwrap().as_str();
                println!("parsing number: {}", number);
                parse_number(number).to_string()
            })
            .to_string();

        // replace space after number with _
        let re = Regex::new(r"\s(\d)\s").unwrap();
        part_name_formatted = re
            .replace_all(&part_name_formatted, |caps: &regex::Captures| {
                format!(" {}_", caps.get(1).unwrap().as_str())
            })
            .to_string();

        // add _ after each preposition
        part_name_formatted = part_name_formatted.replace(" with ", " with_");
        part_name_formatted = part_name_formatted.replace(" on ", " on_");
        part_name_formatted = part_name_formatted.replace(" of ", " of_");
        part_name_formatted = part_name_formatted.replace(" for ", " for_");
        part_name_formatted = part_name_formatted.replace(" in ", " in_");
        part_name_formatted = part_name_formatted.replace(" to ", " to_");
        part_name_formatted = part_name_formatted.replace(" from ", " from_");

        for _ in 0..entry.quantity {
            writeln!(file, "{} {}", color_formatted, part_name_formatted)?;
        }
    }
    Ok(())
}

// create wordcloud from text file
pub fn wordcloud(inventory_entries: Vec<InventoryEntry>, output_path: &str) -> Result<()> {
    text_file(inventory_entries)?;

    // create stopwords file
    let mut file = File::create("temp/stopwords.txt")?;
    writeln!(file, r"\n")?;

    // run wordcloud_cli with text file as input and path as output
    Command::new("wordcloud_cli")
        .arg("--text")
        .arg(TEMP_TEXT_FILE)
        .arg("--imagefile")
        .arg(output_path)
        .arg("--background")
        .arg("#FFFFFF")
        .arg("--color")
        .arg("#134567")
        .arg("--width")
        .arg("1920")
        .arg("--height")
        .arg("1080")
        .arg("--stopwords")
        .arg("temp/stopwords.txt")
        .output()?;
    Ok(())
}

fn parse_number(number_str: &str) -> i32 {
    // turn number to lowercase
    let number_str = number_str.to_lowercase();
    // replace written numbers with numbers
    match number_str.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            // print warning
            println!("Warning: parsing {} is not supported yet", number_str);
            0
        }
    }
}
