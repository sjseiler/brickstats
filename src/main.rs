mod gnuplot_wrapper;
mod plot;
/// A tool for generating lego related diagrams and visualizations.
mod png_plot;
use std::ops::RangeBounds;

use colored::Colorize;
use lazy_static::lazy_static;
use plot::{Bins, Histogram};
use reqwest;
use serde_json::Value;

lazy_static! {
    // read api token from file "../secrets/api_token.txt"
    static ref API_AUTH_TOKEN: Box<String> = Box::new(std::fs::read_to_string("secrets/api_token.txt").unwrap());
}

const CATEGORY_PAGE_SIZE: i32 = 500;
const INVENTORY_PAGE_SIZE: i32 = 100;

// part details
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct part_details {
    part_num: String,
    name: String,
    part_cat_id: i32,
    year_from: i32,
    year_to: i32,
    part_url: String,
    part_img_url: Option<String>,
    prints: Vec<String>,
    molds: Vec<String>,
    alternates: Vec<String>,
    // external_ids: String,
    print_of: Option<String>,
}

// rebrickable database objects
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct part_relationship {
    id: i32,
    parent_part_num: String,
    child_part_num: String,
    quantity: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct part {
    id: i32,
    part_num: String,
    name: String,
    part_cat_id: i32,
    part_url: String,
    part_img_url: String,
    external_ids: String,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct element {
    element_id: String,
    part_num: String,
    color_id: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct part_category {
    id: i32,
    name: String,
    part_count: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct color {
    id: i32,
    name: String,
    rgb: String,
    is_trans: bool,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct inventory_part {
    id: i32,
    set_num: String,
    part_num: String,
    color_id: i32,
    quantity: i32,
    is_spare: bool,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct inventory {
    id: i32,
    set_num: String,
    version: i32,
    inventory_url: String,
    last_modified_dt: String,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct inventory_minifig {
    inventory_id: i32,
    fig_num: String,
    quantity: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct minifig {
    fig_num: String,
    name: String,
    num_parts: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct inventory_set {
    inventory_id: i32,
    set_num: String,
    quantity: i32,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct set {
    id: i32,
    set_num: String,
    name: String,
    year: i32,
    theme_id: i32,
    num_parts: i32,
    set_img_url: String,
    set_url: String,
    last_modified_dt: String,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
struct theme {
    id: i32,
    name: String,
    parent_id: i32,
    theme_url: String,
    num_sets: i32,
    num_parts: i32,
    theme_img_url: String,
    last_modified_dt: String,
}

// get color rgb values for a vector of inventory_parts as a vector of
impl color {
    pub fn get_all() -> Vec<color> {
        // get list of part categories from /api/v3/lego/part_categories/
        // http request
        let url = format!(
            "https://rebrickable.com/api/v3/lego/colors/?page_size={}&ordering=name&key={}",
            1000, *API_AUTH_TOKEN
        );
        println!("Downloading {}", url);
        let response = reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
        if response.status() != 200 {
            panic!("Error downloading {}", url);
        }
        let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let mut colors = Vec::new();
        for color in json["results"].as_array().unwrap() {
            let id = color["id"].as_i64().unwrap() as i32;
            let name = color["name"].as_str().unwrap().to_string();
            let is_trans = color["is_trans"].as_bool().unwrap();
            let rgb = color["rgb"].as_str().unwrap().to_string();

            colors.push(color {
                id,
                rgb,
                is_trans,
                name,
            });
        }

        colors
    }
}

impl part_category {
    pub fn get_all() -> Vec<part_category> {
        // get list of part categories from /api/v3/lego/part_categories/
        // http request
        let url = format!(
            "https://rebrickable.com/api/v3/lego/part_categories/?page_size={}&ordering=name&key={}",
            CATEGORY_PAGE_SIZE, *API_AUTH_TOKEN
        );
        println!("Downloading {}", url);
        let response = reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
        if response.status() != 200 {
            panic!("Error downloading {}", url);
        }
        let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let mut part_categories = Vec::new();
        for category in json["results"].as_array().unwrap() {
            let id = category["id"].as_i64().unwrap() as i32;
            let name = category["name"].as_str().unwrap().to_string();
            let part_count = category["part_count"].as_i64().unwrap() as i32;
            part_categories.push(part_category {
                id,
                name,
                part_count,
            });
        }
        part_categories
    }
}

// set inventory operations
impl inventory {
    fn new(set_num: &str) -> inventory {
        // check if set_num has 3 to 5 digits without leading zero followed by a dash and 1 digit
        if !set_num.contains("-")
            || set_num.len() < 5
            || set_num.len() > 7
            || set_num.starts_with("0")
            || set_num.chars().nth(set_num.len() - 2).unwrap() != '-'
            || set_num.chars().nth(set_num.len() - 1).unwrap() < '1'
            || set_num.chars().nth(set_num.len() - 1).unwrap() > '9'
        {
            panic!("Invalid set_num: {}", set_num);
        }
        let inventory_url = format!(
            "https://rebrickable.com/api/v3/lego/sets/{}/parts/",
            set_num
        );
        let last_modified_dt = String::from("2020-01-01");
        inventory {
            id: 0,
            set_num: String::from(set_num),
            version: 0,
            inventory_url,
            last_modified_dt,
        }
    }
    fn download(&self) -> Vec<inventory_part> {
        let mut inventory_parts = Vec::new();
        let mut page = 1;
        loop {
            let url = format!(
                "{}?page={}&page_size={}&ordering=color&key={}&inc_minifig_parts=1",
                self.inventory_url, page, INVENTORY_PAGE_SIZE, *API_AUTH_TOKEN
            );
            println!("Downloading {}", url);

            let response =
                reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
            if response.status() != 200 {
                break;
            }

            // get response body
            let response_text = response.text().expect(&format!("Error reading {}", url));
            let response_json: serde_json::Value = match serde_json::from_str(&response_text) {
                Ok(v) => v,
                Err(e) => {
                    panic!("Error parsing response text {}: {}", response_text, e);
                }
            };
            let results = response_json["results"]
                .as_array()
                .expect(&format!("Error parsing json {}", response_json));
            if results.len() == 0 {
                break;
            }
            for result in results {
                let part_num = result["part"]["part_num"]
                    .as_str()
                    .expect(&format!("Error parsing part_num {}", result))
                    .to_string();
                let color_id = result["color"]["id"]
                    .as_i64()
                    .expect(&format!("Error parsing color_id {}", result))
                    as i32;
                let quantity = result["quantity"]
                    .as_i64()
                    .expect(&format!("Error parsing quantity {}", result))
                    as i32;
                let is_spare = result["is_spare"]
                    .as_bool()
                    .expect(&format!("Error parsing is_spare {}", result));
                inventory_parts.push(inventory_part {
                    id: 0,
                    set_num: self.set_num.clone(),
                    part_num,
                    color_id,
                    quantity,
                    is_spare,
                });
            }
            page += 1;
        }
        inventory_parts
    }
}

impl part_details {
    // get part details from rebrickable
    fn new(part: &inventory_part) -> Self {
        let url = format!(
            "https://rebrickable.com/api/v3/lego/parts/{}/?key={}",
            part.part_num, *API_AUTH_TOKEN
        );
        println!("Downloading {}", url);
        let response = reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
        if response.status() != 200 {
            println!("response: {:#?}", response);
            panic!("Error downloading {}", url);
        }
        // get response body
        let response_text = response.text().expect(&format!("Error reading {}", url));
        let response_json: serde_json::Value = match serde_json::from_str(&response_text) {
            Ok(v) => v,
            Err(e) => {
                panic!("Error parsing response text {}: {}", response_text, e);
            }
        };
        // println!("Parsing {}", response_text);

        let part_details = part_details {
            part_num: response_json["part_num"]
                .as_str()
                .expect(&format!("Error parsing part_num {}", response_json))
                .to_string(),
            name: response_json["name"]
                .as_str()
                .expect(&format!("Error parsing name {}", response_json))
                .to_string(),
            part_cat_id: response_json["part_cat_id"]
                .as_i64()
                .expect(&format!("Error parsing part_cat_id {}", response_json))
                as i32,
            year_from: response_json["year_from"]
                .as_i64()
                .expect(&format!("Error parsing year {}", response_json))
                as i32,
            year_to: response_json["year_to"]
                .as_i64()
                .expect(&format!("Error parsing year {}", response_json))
                as i32,
            part_url: response_json["part_url"]
                .as_str()
                .expect(&format!("Error parsing part_url {}", response_json))
                .to_string(),
            part_img_url: match response_json["part_img_url"] {
                serde_json::Value::Null => None,
                _ => Some(
                    response_json["part_img_url"]
                        .as_str()
                        .expect(&format!("Error parsing part_img_url {}", response_json))
                        .to_string(),
                ),
            },
            prints: response_json["prints"]
                .as_array()
                .expect(&format!("Error parsing prints {}", response_json))
                .iter()
                .map(|print| print.to_string())
                .collect(),
            molds: response_json["molds"]
                .as_array()
                .expect(&format!("Error parsing molds {}", response_json))
                .iter()
                .map(|mold| mold.to_string())
                .collect(),
            alternates: response_json["alternates"]
                .as_array()
                .expect(&format!("Error parsing alternates {}", response_json))
                .iter()
                .map(|alternate| alternate.to_string())
                .collect(),
            print_of: match response_json["print_of"] {
                serde_json::Value::Null => None,
                _ => Some(
                    response_json["print_of"]
                        .as_str()
                        .expect(&format!("Error parsing print_of {}", response_json))
                        .to_string(),
                ),
            },
        };
        part_details
    }

    // print part details nicely formatted
    fn print(&self) {
        println!("Part: {}", self.part_num);
        println!("Name: {}", self.name);
        println!("Part Category: {}", self.part_cat_id);
        println!("Year: {} - {}", self.year_from, self.year_to);
        println!("Part URL: {}", self.part_url);
        println!("Part Image URL: {:?}", self.part_img_url);
        println!("Prints: {}", self.prints.join(", "));
        println!("Molds: {}", self.molds.join(", "));
        println!("Alternates: {}", self.alternates.join(", "));
        println!("Print of: {:?}", self.print_of);
    }

    // calculate average age of parts in inventory
    fn average_part_year(
        inventory_parts: &Vec<inventory_part>,
        part_details: &Vec<part_details>,
    ) -> f32 {
        let mut average_year = 0.0;
        let mut part_count = 0;

        // zip inventory_parts and part_details
        let tuples = inventory_parts.iter().zip(part_details.iter());

        for (inventory_part, part_details) in tuples {
            average_year += part_details.year_from as f32 * inventory_part.quantity as f32;
            part_count += inventory_part.quantity;
        }
        average_year / part_count as f32
    }

    // get many part_details at once
    fn get_many(part_numbers: &Vec<String>) -> Vec<part_details> {
        // split part_numbers into chunks of 100
        let mut part_details = Vec::new();
        for chunk in part_numbers.chunks(100) {
            part_details.append(&mut part_details::get_100(&chunk.to_vec()));
        }
        part_details
    }

    fn get_100(part_numbers: &Vec<String>) -> Vec<part_details> {
        println!("Getting part details for {} parts", part_numbers.len());
        let url = format!(
            "https://rebrickable.com/api/v3/lego/parts/?key={}&part_nums={}&inc_part_details=1",
            *API_AUTH_TOKEN,
            part_numbers.join(",")
        );
        println!("Downloading {}", url);
        let response = reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
        if response.status() != 200 {
            println!("response: {:#?}", response);
            panic!("Error downloading {}", url);
        }
        // get response body
        let response_text = response.text().expect(&format!("Error reading {}", url));
        let response_json: serde_json::Value = match serde_json::from_str(&response_text) {
            Ok(v) => v,
            Err(e) => {
                panic!("Error parsing response text {}: {}", response_text, e);
            }
        };
        // println!("Parsing {}", response_text);

        let many_part_details = response_json["results"]
            .as_array()
            .expect(&format!("Error parsing results {}", response_json))
            .iter()
            .map(|result| {
                // println!("Parsing {}", result);
                let part_details = part_details {
                    part_num: result["part_num"]
                        .as_str()
                        .expect(&format!("Error parsing part_num {}", result))
                        .to_string(),
                    name: result["name"]
                        .as_str()
                        .expect(&format!("Error parsing name {}", result))
                        .to_string(),
                    part_cat_id: result["part_cat_id"]
                        .as_i64()
                        .expect(&format!("Error parsing part_cat_id {}", result))
                        as i32,
                    year_from: result["year_from"]
                        .as_i64()
                        .expect(&format!("Error parsing year {}", result))
                        as i32,
                    year_to: result["year_to"]
                        .as_i64()
                        .expect(&format!("Error parsing year {}", result))
                        as i32,
                    part_url: result["part_url"]
                        .as_str()
                        .expect(&format!("Error parsing part_url {}", result))
                        .to_string(),
                    part_img_url: match result["part_img_url"] {
                        serde_json::Value::Null => None,
                        _ => Some(
                            result["part_img_url"]
                                .as_str()
                                .expect(&format!("Error parsing part_img_url {}", result))
                                .to_string(),
                        ),
                    },
                    prints: result["prints"]
                        .as_array()
                        .expect(&format!("Error parsing prints {}", result))
                        .iter()
                        .map(|print| print.to_string())
                        .collect(),
                    molds: result["molds"]
                        .as_array()
                        .expect(&format!("Error parsing molds {}", result))
                        .iter()
                        .map(|mold| mold.to_string())
                        .collect(),
                    alternates: result["alternates"]
                        .as_array()
                        .expect(&format!("Error parsing alternates {}", result))
                        .iter()
                        .map(|alternate| alternate.to_string())
                        .collect(),
                    print_of: match result["print_of"] {
                        serde_json::Value::Null => None,
                        _ => Some(
                            result["print_of"]
                                .as_str()
                                .expect(&format!("Error parsing print_of {}", result))
                                .to_string(),
                        ),
                    },
                };
                part_details
            })
            .collect();
        many_part_details
    }
}

impl inventory_part {
    // print nicely formatted inventory
    #[allow(dead_code)]
    fn print(inventory_parts: &Vec<inventory_part>) {
        let mut part_numbers = Vec::new();
        let mut part_colors = Vec::new();
        let mut part_quantities = Vec::new();
        for inventory_part in inventory_parts {
            part_numbers.push(inventory_part.part_num.clone());
            part_colors.push(inventory_part.color_id);
            part_quantities.push(inventory_part.quantity);
        }
        let mut part_numbers = part_numbers
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        part_numbers.sort();
        part_numbers.dedup();
        let mut part_colors = part_colors
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        part_colors.sort();
        part_colors.dedup();
        let mut part_quantities = part_quantities
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        part_quantities.sort();
        part_quantities.dedup();
        println!(
            "{} {} {} {}",
            "Part".green(),
            "Color".green(),
            "Quantity".green(),
            "Spare".green()
        );
        for part_number in part_numbers {
            for part_color in &part_colors {
                let mut quantity = 0;
                let mut is_spare = false;
                for inventory_part in inventory_parts {
                    if inventory_part.part_num == part_number
                        && inventory_part.color_id.to_string() == *part_color
                    {
                        quantity += inventory_part.quantity;
                        is_spare = inventory_part.is_spare;
                    }
                }
                println!(
                    "{} {} {} {}",
                    part_number,
                    part_color,
                    quantity,
                    if is_spare { "yes" } else { "no" }
                );
            }
        }
    }
}

impl part {
    // get part from rebrickable
    #[allow(dead_code)]
    fn get_part(part_num: &str) -> part {
        let url = format!(
            "https://rebrickable.com/api/v3/lego/parts/{}/?key={}",
            part_num, *API_AUTH_TOKEN
        );
        println!("Downloading {}", url);
        let response = reqwest::blocking::get(&url).expect(&format!("Error downloading {}", url));
        if response.status() != 200 {
            panic!("Error downloading {}", url);
        }
        // get response body
        let response_text = response.text().expect(&format!("Error reading {}", url));
        let response_json: serde_json::Value = match serde_json::from_str(&response_text) {
            Ok(v) => v,
            Err(e) => {
                panic!("Error parsing response text {}: {}", response_text, e);
            }
        };
        let part_num = response_json["part_num"]
            .as_str()
            .expect(&format!("Error parsing part_num {}", response_json))
            .to_string();
        let name = response_json["name"]
            .as_str()
            .expect(&format!("Error parsing name {}", response_json))
            .to_string();
        let part_url = response_json["part_url"]
            .as_str()
            .expect(&format!("Error parsing part_url {}", response_json))
            .to_string();
        let part_img_url = response_json["part_img_url"]
            .as_str()
            .expect(&format!("Error parsing part_img_url {}", response_json))
            .to_string();
        let external_ids = response_json["external_ids"]
            .as_array()
            .expect(&format!("Error parsing external_ids {}", response_json))
            .iter()
            .map(|external_id| {
                external_id["external_id"]
                    .as_str()
                    .expect(&format!("Error parsing external_id {}", external_id))
                    .to_string()
            })
            .collect();
        let part_cat_id = response_json["part_cat_id"]
            .as_i64()
            .expect(&format!("Error parsing part_cat_id {}", response_json))
            as i32;
        part {
            id: 0,
            part_num,
            name,
            part_url,
            part_img_url,
            external_ids,
            part_cat_id,
        }
    }
}

// download and parse the rebrickable database
// https://rebrickable.com/downloads/
// https://rebrickable.com/api/v3/docs/

// download set inventory from rebrickable
// https://rebrickable.com/api/v3/docs/#operation/get_sets__set_num__parts

fn main() {
    // get set num from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: lego <set_num>");
    }
    let set_num = &args[1];
    println!("set_num: {}", set_num);

    download_plot(set_num);
}

fn download_plot(set_num: &str) {
    // download set inventory
    let inventory = inventory::new(set_num);
    let inventory_parts = inventory.download();

    // get part details for all parts
    let all_part_details =
        part_details::get_many(&inventory_parts.iter().map(|p| p.part_num.clone()).collect());
    println!("Got part details for {} parts", all_part_details.len());

    let category_details = part_category::get_all();
    let colors = color::get_all();

    // create new vector with (part_category_id, quantity, color_id) tuples
    let mut data_tuples: Vec<(i32, i32, i32)> = Vec::new();
    // for all inventory_parts
    for inventory_part in &inventory_parts {
        // find the part_category_id for the part by part_num
        let part_category_id = match all_part_details
            .iter()
            .find(|part_details| part_details.part_num == inventory_part.part_num)
        {
            Some(part_details) => part_details.part_cat_id,
            None => {
                println!(
                    "Error finding part_category_id for part_num {}",
                    inventory_part.part_num
                );
                0
            }
        };
        data_tuples.push((
            part_category_id,
            inventory_part.quantity,
            inventory_part.color_id,
        ));
    }

    // sort data tuples by color_id
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
            .find(|category_details| category_details.id == *a)
        {
            Some(category_details) => category_details.name.clone(),
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
            .find(|category_details| category_details.id == *b)
        {
            Some(category_details) => category_details.name.clone(),
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
            .find(|category_details| category_details.id == *part_category_id)
        {
            Some(category_details) => category_details.name.clone(),
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
        let rgb = match colors.iter().find(|color| color.id == id) {
            Some(color) => color.rgb.clone(),
            None => {
                println!("Error finding rgb for color_id {}", id);
                "000000".to_string()
            }
        };
        color_rgbs.push(rgb);
    }

    // print color_rgbs
    println!("color_rgbs: {:?}", color_rgbs);

    gnuplot_wrapper::Gnuplot::show(
        &format!(
            "{}\nset title \"Parts of Set {set_num}",
            gnuplot_wrapper::DEFAULT_CONFIG
        ),
        labels,
        data,
        color_rgbs,
    )
    .unwrap();
}

// download inventory and plot it in histogram
fn download_plot_shell(set_num: &str) {
    // download set inventory
    let inventory = inventory::new(set_num);
    let inventory_parts = inventory.download();

    // print inventory
    // inventory_part::print(&inventory_parts);

    // get the details for the first part
    let part_num = inventory_parts[0].clone();
    let part_details = part_details::new(&part_num);
    part_details.print();

    // print part details for all parts
    let all_part_details =
        part_details::get_many(&inventory_parts.iter().map(|p| p.part_num.clone()).collect());
    println!("Got part details for {} parts", all_part_details.len());
    // for part_details in &all_part_details {
    //     part_details.print();
    // }
    let avg_year = part_details::average_part_year(&inventory_parts, &all_part_details);
    println!("avg_year: {}", avg_year);

    // // extract list of all part_categories and remove duplicates
    // let mut all_part_categories = Vec::new();
    // for part_details in &all_part_details {
    //     if !all_part_categories.contains(&part_details.part_cat_id) {
    //         all_part_categories.push(part_details.part_cat_id);
    //     }
    // }

    // get the part_categories lookup table from rebrickable
    let category_details = part_category::get_all();
    // for category_details in &category_details {
    //     println!("category_details: {:?}", category_details);
    // }

    // create new vector with (part_category_id, quantity) tuples
    let mut data_tuples: Vec<(i32, i32)> = Vec::new();
    // for all inventory_parts
    for inventory_part in &inventory_parts {
        // find the part_category_id for the part by part_num
        let part_category_id = match all_part_details
            .iter()
            .find(|part_details| part_details.part_num == inventory_part.part_num)
        {
            Some(part_details) => part_details.part_cat_id,
            None => {
                println!(
                    "Error finding part_category_id for part_num {}",
                    inventory_part.part_num
                );
                0
            }
        };

        // if data already contains a Data with the same label, add the quantity to the existing Data
        if let Some(tuple) = data_tuples
            .iter_mut()
            .find(|data| data.0 == part_category_id)
        {
            tuple.1 += inventory_part.quantity;
        } else {
            // otherwise create a new Data with the label and quantity
            data_tuples.push((part_category_id, inventory_part.quantity));
        }
    }

    // create empty data vector
    let mut data: Vec<plot::Data> = Vec::new();

    // look-up the part_category names
    for (part_category_id, quantity) in &data_tuples {
        let part_category_name = category_details
            .iter()
            .find(|category_details| category_details.id == *part_category_id)
            .unwrap()
            .name
            .clone();
        data.push(plot::Data::new_with_label(
            *quantity as f64,
            &part_category_name,
        ));
    }

    let bins = Bins::new_from_labels(&data);

    let histogram = Histogram::new(&data, bins);
    histogram.plot_weighted();
}
