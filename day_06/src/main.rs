mod read;
mod lib;

use lib::{CoordinateUnit, CoordinatePoint, Map};

const PART_2_THRESHOLD : CoordinateUnit = 10000; 

fn main() {

    let coordinates_string = read::read_lines("input.txt".to_owned());
    let coordinates : Vec<CoordinatePoint> = coordinates_string.into_iter()
        .map(|coordinate_string| {
            let mut coordinate_split = coordinate_string.split(", ");
            let coordinate_x : CoordinateUnit = coordinate_split.next().unwrap().parse().unwrap();
            let coordinate_y : CoordinateUnit = coordinate_split.next().unwrap().parse().unwrap();
            return CoordinatePoint::new(coordinate_x, coordinate_y);
        }).collect();
    let mut map : Map = Map::new(coordinates);
    map.compute_mapping();
    
    // Part 1
    let mapping = map.get_area_for_limitted();
    let max = mapping.into_iter().max_by_key(|(_, area)| *area).unwrap();
    println!("\r🛰️  Max area of '{}' from non-infinite '{}' section (Part 1)", max.1, max.0);
    
    // Part 2
    let region_of_interest = map.get_points_with_sum_less(PART_2_THRESHOLD);
    println!("\r🛰️  The region of interest for threshold '{}' has '{}' points (Part 2)", PART_2_THRESHOLD, region_of_interest.len());
}