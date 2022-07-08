mod read;
mod lib;

use lib::SystemDecoder;
fn main() {

    let system_codes = read::read_list_int_lines("input.txt".to_owned(), " ");
    let mut system_decoder = SystemDecoder::new(system_codes.get(0).unwrap().clone());
    system_decoder.decode_codes();
    
    // Part 1
    let sum_metadata = system_decoder.sum_metadata();
    println!("\r💻  Sum of nodes metadata: '{}' (Part 1)", sum_metadata);
    
    // Part 2
    let root_node_value = system_decoder.get_root_value();
    println!("\r💻  Root node value: '{}' (Part 2)", root_node_value);
}