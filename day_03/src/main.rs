mod read;
mod lib;

use lib::{Claim, FabricManager};

fn main() {

    let claim_codes = read::read_lines("input.txt".to_owned());
    let claims : Vec<Claim> = claim_codes.into_iter().map(|code| Claim::new(code)).collect();
    let mut fabric_manager : FabricManager = FabricManager::new(claims);

    fabric_manager.develop_mapping();


    // Part 1
    let number_of_conflict_positions = fabric_manager.check_mapping_position_equal_or_higher(2);
    println!("\r👔 Number of positions in conflict: '{}' (Part 1)", number_of_conflict_positions);
    
    // Part 2
    let claim_without_conflict = fabric_manager.get_claim_without_conflicts().unwrap();
    println!("\r👔 Claim without conflicts: '{}' (Part 2)", claim_without_conflict);
}