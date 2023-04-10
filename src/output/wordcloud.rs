use anyhow::Result;
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
        let part_name_formatted = entry.part_name.replace(" x ", "x");

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
