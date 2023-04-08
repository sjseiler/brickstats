mod plot;
mod rebrickable;
mod stats;
/// A tool for generating lego related diagrams and visualizations.
use std::fs::read_to_string;

use rebrickable::{color, inventory, part_category, part_details};

fn main() {
    // read api token from file "../secrets/api_token.txt"
    let api_token =
        read_to_string("secrets/api_token.txt").expect("Couldn't read api token from file");

    // get set num from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: lego <set_num>");
    }
    let set_num = &args[1];
    println!("set_num: {}", set_num);

    download_plot(set_num, &api_token);
}

fn download_plot(set_num: &str, api_token: &str) {
    // download set inventory
    let inventory = inventory::new(set_num);
    let inventory_parts = inventory.download(api_token);

    // get part details for all parts
    let all_part_details = part_details::get_many(
        inventory_parts.iter().map(|p| p.get_part_num()).collect(),
        api_token,
    );
    println!("Got part details for {} parts", all_part_details.len());

    let category_details = part_category::get_all(api_token);
    let colors = color::get_all(api_token);

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

    // print color_rgbs
    println!("color_rgbs: {:?}", color_rgbs);

    plot::Gnuplot::show(
        &format!(
            "{}\nset title \"Parts of Set {set_num} (including spares)",
            plot::DEFAULT_CONFIG
        ),
        labels,
        data,
        color_rgbs,
    )
    .unwrap();
}
