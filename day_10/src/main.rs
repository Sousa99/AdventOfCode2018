use regex::Regex;

mod read;
mod lib;

use lib::{Coordinate2D, PointDefinition, Sky};

fn main() {

    let point_definitions_string = read::read_lines("input.txt".to_owned());

    let point_definition_regex : Regex = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let point_definitions : Vec<PointDefinition> = point_definitions_string.into_iter()
        .map(|point_definition| {
            let point_capture = point_definition_regex.captures(&point_definition).unwrap();

            let position_x : i64 = point_capture.get(1).unwrap().as_str().parse().unwrap();
            let position_y : i64 = point_capture.get(2).unwrap().as_str().parse().unwrap();
            let velocity_x : i64 = point_capture.get(3).unwrap().as_str().parse().unwrap();
            let velocity_y : i64 = point_capture.get(4).unwrap().as_str().parse().unwrap();

            let position : Coordinate2D = Coordinate2D { x: position_x, y: position_y };
            let velocity : Coordinate2D = Coordinate2D { x: velocity_x, y: velocity_y };

            return PointDefinition { position: position, velocity: velocity }; })
        .collect();
    let mut sky : Sky = Sky::new(&point_definitions, 10, 0.25);
    
    while sky.worth_running() {
        sky.run_iteration();
        let current_iteration = sky.get_current_iteration();
        
        if sky.worth_printing() {
            let current_sky = sky.print_current_sky();

            println!("🌟 Iteration: {}", current_iteration);
            println!("{}", current_sky);
            println!();
        }
    }
}