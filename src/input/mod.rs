mod rebrickable;

use crate::output::Dataset;
pub use rebrickable::{color, inventory, inventory_part, part_category, part_details};
use serde::{de, Deserialize, Serialize};
use std::path::Path;

pub struct Rebrickable{
    api_token: String,
}

impl Rebrickable {
    pub fn new(api_token: String) -> Rebrickable {
        Rebrickable { api_token }
    }

    pub fn inventory(&self, set_num: &str) -> Vec<inventory_part> {
        // download set inventory
        let inventory = inventory::new(set_num);
        inventory.download(&self.api_token, false)
    }

    pub fn all_colors(&self) -> Vec<color> {
        color::get_all_cached(&self.api_token)
    }

    pub fn all_categories(&self) -> Vec<part_category> {
        part_category::get_all_cached(&self.api_token)
    }

    pub fn part_details(&self, inventory_parts: &[inventory_part]) -> Vec<part_details> {
        // get all part details for the parts in the inventory
        let all_part_details = part_details::get_many(
            inventory_parts.iter().map(|p| p.part_num()).collect(),
            &self.api_token,
        );
        println!("Got part details for {} parts", all_part_details.len());
        all_part_details
    }
}

pub fn prepare_dataset(inventory_parts: Vec<inventory_part>, part_details: Vec<part_details>, categories: Vec<part_category>, colors: Vec<color>) -> Dataset {   

    // create new vector with (part_category_id, quantity, color_id) tuples
    let mut data_tuples: Vec<(i32, i32, i32)> = Vec::new();
    // for all inventory_parts
    for inventory_part in &inventory_parts {
        // find the part_category_id for the part by part_num
        let part_category_id = match part_details
            .iter()
            .find(|part_details| part_details.part_num() == inventory_part.part_num())
        {
            Some(part_details) => part_details.part_cat_id(),
            None => {
                println!(
                    "Error finding part_category_id for part_num {}",
                    inventory_part.part_num()
                );
                0
            }
        };
        data_tuples.push((
            part_category_id,
            inventory_part.quantity(),
            inventory_part.color_id(),
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
        let a_name = match categories
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
        let b_name = match categories
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
        let part_category_name = match categories
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
        let rgb = match colors.iter().find(|color| color.id() == id) {
            Some(color) => color.rgb(),
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

pub fn inventory_from_file(path: &str) -> Vec<inventory_part> {
    // check if path exists and open file
    let path = Path::new(path);
    if !path.exists() {
        panic!("Error: file {} does not exist", path.display());
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
        inventory_parts
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryEntry {
    color: String,
    part_name: String,
    quantity: i32,
}

pub fn formatted_rebrickable_inventory(
    set_num: &str,
    api_token: &str,
    path: &str,
) -> Vec<InventoryEntry> {
    let inventory_parts = inventory::new(set_num).download(api_token, false);

    // get part details for all inventory parts
    let all_part_details = part_details::get_many(
        inventory_parts.iter().map(|p| p.part_num()).collect(),
        api_token,
    );

    let all_colors = color::get_all_cached(api_token);

    // create inventory entries for all inventory parts
    let mut inventory_entries: Vec<InventoryEntry> = Vec::new();
    for inventory_part in inventory_parts {
        let part_details = all_part_details
            .iter()
            .find(|part_details| part_details.part_num() == inventory_part.part_num())
            .unwrap();

        // find color name in colors
        let color_name = all_colors
            .iter()
            .find(|color| color.id() == inventory_part.color_id())
            .unwrap()
            .name();

        inventory_entries.push(InventoryEntry {
            color: color_name,
            part_name: part_details.name(),
            quantity: inventory_part.quantity(),
        });
    }
    inventory_entries
}
