mod read;
mod lib;

use lib::Warehouse;

fn main() {

    let ids = read::read_lines("input.txt".to_owned());
    let warehouse = Warehouse::new(ids);

    // Part 1
    let number_exact_two = warehouse.get_number_verify_exact(2);
    let number_exact_three = warehouse.get_number_verify_exact(3);
    let checksum = number_exact_two * number_exact_three;
    println!("\r📦 Warehouse checksum result: '{}' x '{}' = '{}' (Part 1)", number_exact_two, number_exact_three, checksum);
    
    // Part 2
    let matched_ids = warehouse.get_matched_boxes(1).unwrap();
    println!("\r📦 Warehouse matched: '{}' (Part 2)", matched_ids);
}