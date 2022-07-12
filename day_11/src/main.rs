mod read;
mod lib;

use lib::{GridSerialNumber, Coordinate2D, FuelGrid};

fn main() {

    let grid_serial_number : GridSerialNumber = *read::read_int_lines("input.txt".to_owned()).get(0).unwrap() as GridSerialNumber;

    let top_left_coordinate : Coordinate2D = Coordinate2D::new(1, 1);
    let bottom_right_coordinate : Coordinate2D = Coordinate2D::new(300, 300);

    let mut fuel_grid : FuelGrid = FuelGrid::new(top_left_coordinate, bottom_right_coordinate, grid_serial_number);
    fuel_grid.compute_fuel_levels();

    // println!("{}", fuel_grid._print_formatted());
    // println!();

    // Part 1
    let max_fuel_grid = fuel_grid.get_max_fuel_square_size(3);
    println!("⚡ Max fuel level found in '({}, {})' with value of '{}' (Part 1)", max_fuel_grid.0.get_x(), max_fuel_grid.0.get_y(), max_fuel_grid.1);
    
    // Part 2
    let max_fuel_grid = fuel_grid.get_max_fuel_square_any();
    println!("⚡ Max fuel level found in '({}, {}, {})' with value of '{}' (Part 1)", max_fuel_grid.0.get_x(), max_fuel_grid.0.get_y(), max_fuel_grid.1, max_fuel_grid.2);
}