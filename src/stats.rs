use crate::rebrickable::{inventory_part, part_details};

// calculate average age of parts in inventory
#[allow(dead_code)]
pub fn average_part_year(inventory_parts: &[inventory_part], part_details: &[part_details]) -> f32 {
    let mut average_year = 0.0;
    let mut part_count = 0;

    // zip inventory_parts and part_details
    let tuples = inventory_parts.iter().zip(part_details.iter());

    for (inventory_part, part_details) in tuples {
        average_year += part_details.get_year_from() as f32 * inventory_part.get_quantity() as f32;
        part_count += inventory_part.get_quantity();
    }
    average_year / part_count as f32
}
