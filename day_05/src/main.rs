mod read;
mod lib;

use lib::{Polymer, ProblematicPolymer};

fn main() {

    let polymers_chars = read::read_chars("input.txt".to_owned());
    let polymer_chars = polymers_chars.get(0).unwrap();
    
    // Part 1
    let mut polymer = Polymer::new(polymer_chars);
    let mut changed = true;
    while changed { changed = polymer.run_iteration(); }
    let last_polymer = polymer.get_last_iteration();
    println!("\r🧫 Last polymer has '{}' units (Part 1)", last_polymer.len());
    
    // Part 2
    let mut problematic_polymer = ProblematicPolymer::new(polymer_chars);
    problematic_polymer.run_polymers();
    let problematic_result = problematic_polymer.get_less_problematic().unwrap();
    println!("\r🧫 Less problematic polymer found for '{}' has '{}' units (Part 2)", problematic_result.0, problematic_result.1.len());
}