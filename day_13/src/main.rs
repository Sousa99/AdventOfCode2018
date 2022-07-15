mod read;
mod lib;

use lib::{Coordinate2D, Simulator};

fn main() {

    let input_read : Vec<Vec<char>> = read::read_chars("input.txt".to_owned());
    let mut simulator : Simulator = Simulator::new(input_read);

    // Part 1
    //println!("{}\n", simulator._print_map(true));
    while simulator.get_crashes().len() == 0 {
        simulator.run_iteration();
        //println!("{}\n", simulator._print_map(true));
    }
    let crashes : Vec<(&Coordinate2D, &Vec<usize>)> = simulator.get_crashes();
    let iteration : usize = simulator.get_iteration();
    let crash : &(&Coordinate2D, &Vec<usize>) = crashes.get(0).unwrap();
    println!("🛒 The first crash was registered after '{}' iterations, at '({}, {})' (Part 1)", iteration, crash.0.get_x(), crash.0.get_y());

    // Part 2
    //println!("{}\n", simulator._print_map(false));
    while simulator.get_carts_positions().len() != 1 {
        simulator.run_iteration();
        //println!("{}\n", simulator._print_map(false));
    }
    let remaining_carts : Vec<&Coordinate2D> = simulator.get_carts_positions();
    let iteration : usize = simulator.get_iteration();
    let cart_position : &Coordinate2D = remaining_carts.get(0).unwrap();
    println!("🛒 The last cart is at '({}, {})' after '{}' iterations (Part 2)", cart_position.get_x(), cart_position.get_y(), iteration);
    
}