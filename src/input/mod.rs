mod rebrickable;

use crate::output::Dataset;
pub use rebrickable::{color, inventory, inventory_part, part_category, part_details};
use serde::{de, Deserialize};
use std::path::Path;

pub fn dataset_from_rebrickable(set_num: &str, api_token: &str) -> Dataset {
    // download set inventory
    let inventory = inventory::new(set_num);
    let inventory_parts = inventory.download(api_token, false);

    prepare_dataset(inventory_parts, api_token)
}

fn prepare_dataset(inventory_parts: Vec<inventory_part>, api_token: &str) -> Dataset {
    // get part details for all parts
    let all_part_details = part_details::get_many(
        inventory_parts.iter().map(|p| p.get_part_num()).collect(),
        api_token,
    );
    println!("Got part details for {} parts", all_part_details.len());

    let category_details = part_category::get_all_cached(api_token);
    let colors = color::get_all_cached(api_token);

    // create new vector with (part_category_id, quantity, color_id) tuples
    let mut data_tuples: Vec<(i32, i32, i32)> = Vec::new();
    // for all inventory_parts
    for inventory_part in &inventory_parts {
        // find the part_category_id for the part by part_num
        let part_category_id = match all_part_details
            .iter()
            .find(|part_details| part_details.get_part_num() == inventory_part.get_part_num())
        {
            Some(part_details) => part_details.get_part_cat_id(),
            None => {
                println!(
                    "Error finding part_category_id for part_num {}",
                    inventory_part.get_part_num()
                );
                0
            }
        };
        data_tuples.push((
            part_category_id,
            inventory_part.get_quantity(),
            inventory_part.get_color_id(),
        ));
    }

    // sort by quantity first
    data_tuples.sort_by(|a, b| b.1.cmp(&a.1));
    // then by color_id
    data_tuples.sort_by(|a, b| a.2.cmp(&b.2));

    let mut data: Vec<Vec<i32>> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    let mut color_rgbs: Vec<String> = Vec::new();
    let mut color_ids: Vec<i32> = Vec::new();

    // fill unique part_category_ids
    let mut unique_part_category_ids: Vec<i32> = Vec::new();
    for (part_category_id, _, _) in &data_tuples {
        if !unique_part_category_ids.contains(part_category_id) {
            unique_part_category_ids.push(*part_category_id);
        }
    }

    // sort unique_part_category_ids by category name
    unique_part_category_ids.sort_by(|a, b| {
        let a_name = match category_details
            .iter()
            .find(|category_details| category_details.get_id() == *a)
        {
            Some(category_details) => category_details.get_name(),
            None => {
                println!(
                    "Error finding part_category_name for part_category_id {}",
                    a
                );
                "unknown".to_string()
            }
        };
        let b_name = match category_details
            .iter()
            .find(|category_details| category_details.get_id() == *b)
        {
            Some(category_details) => category_details.get_name(),
            None => {
                println!(
                    "Error finding part_category_name for part_category_id {}",
                    b
                );
                "unknown".to_string()
            }
        };
        a_name.cmp(&b_name)
    });

    // fill data and color_ids with with datatuples values
    for tuple in &data_tuples {
        // get index of part_category_id in unique_part_category_ids
        let index = unique_part_category_ids
            .iter()
            .position(|id| id == &tuple.0)
            .unwrap();

        // get len of unique_part_category_ids
        let len = unique_part_category_ids.len();

        // create new data vector
        let mut new_data: Vec<i32> = vec![0; len];

        // set quantity at index
        new_data[index] = tuple.1;

        // push new_data to data
        data.push(new_data);

        // push color_id to color_ids
        color_ids.push(tuple.2);
    }

    // replace unique_part_category_ids with names
    for part_category_id in &unique_part_category_ids {
        let part_category_name = match category_details
            .iter()
            .find(|category_details| category_details.get_id() == *part_category_id)
        {
            Some(category_details) => category_details.get_name(),
            None => {
                println!(
                    "Error finding part_category_name for part_category_id {}",
                    part_category_id
                );
                "unknown".to_string()
            }
        };
        labels.push(part_category_name);
    }

    // replace color_ids with rgb values
    for id in color_ids {
        let rgb = match colors.iter().find(|color| color.get_id() == id) {
            Some(color) => color.get_rgb(),
            None => {
                println!("Error finding rgb for color_id {}", id);
                "000000".to_string()
            }
        };
        color_rgbs.push(rgb);
    }

    Dataset::new("".to_string(), labels, data, color_rgbs)
}

#[derive(Debug, Deserialize)]
struct DatasetEntry {
    #[serde(rename = "Part")]
    part: String,
    #[serde(rename = "Quantity")]
    quantity: i32,
    #[serde(rename = "Color")]
    color_id: i32,
    #[serde(rename = "Is Spare", deserialize_with = "deserialize_bool", default)]
    is_spare: Option<bool>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_ref() {
        "False" => Ok(Some(false)),
        "True" => Ok(Some(true)),
        _ => Err(serde::de::Error::custom(format!("invalid bool: {}", s))),
    }
}

pub fn dataset_from_file(path: &str, api_token: &str) -> Dataset {
    // check if path exists and open file
    let path = Path::new(path);
    if !path.exists() {
        // output error message
        println!("Error: file {} does not exist", path.display());
        // return empty dataset
        Dataset::new("".to_string(), Vec::new(), Vec::new(), Vec::new())
    } else {
        // read csv file with columns Part,Color,Quantity,Is Spare
        let mut rdr = csv::Reader::from_path(path).unwrap();

        let mut inventory_parts = Vec::new();

        // read csv file into data
        for result in rdr.deserialize() {
            let record: DatasetEntry = result.unwrap();

            if record.is_spare == Some(true) {
                continue;
            }

            inventory_parts.push(inventory_part::new_simplified(
                record.part,
                record.color_id,
                record.quantity,
                true,
            ));
        }

        prepare_dataset(inventory_parts, api_token)
    }
}
