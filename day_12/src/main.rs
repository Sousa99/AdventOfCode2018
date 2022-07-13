use regex::Regex;

mod read;
mod lib;

use lib::{Pot, Rule, Garden};

const FIRST_PART_NUMBER_ITERATIONS : usize = 20;
const SECOND_PART_NUMBER_ITERATIONS : usize = 50000000000;

fn main() {

    let mut input_read : Vec<String> = read::read_lines("input.txt".to_owned());
    let initial_state_line = input_read.remove(0);
    input_read.remove(0);
    let rule_lines = input_read;

    let initial_state_regex : Regex = Regex::new(r"initial state: ([#|\.]+)").unwrap();
    let rule_regex : Regex = Regex::new(r"([#|\.]+) => ([#|\.])").unwrap();

    let initial_pots : Vec<Pot> = initial_state_regex.captures(&initial_state_line)
        .unwrap().get(1)
        .unwrap().as_str().chars().into_iter()
        .map(|pot_state_char| Pot::new(pot_state_char))
        .collect();
    let rules : Vec<Rule> = rule_lines.iter()
        .map(|rule_string| rule_regex.captures(rule_string).unwrap())
        .map(|rule_capture| (rule_capture.get(1).unwrap().as_str(), rule_capture.get(2).unwrap().as_str()))
        .map(|(rule_requisit_str, rule_result_str)| Rule::new(rule_requisit_str.chars().collect(), rule_result_str.chars().nth(0).unwrap()))
        .collect();

    let mut garden : Garden = Garden::new(initial_pots, rules);
        
    // Part 1
    garden.run_until_iteration(FIRST_PART_NUMBER_ITERATIONS);
    let sum_current_position = garden.get_iteration_sum_with_plants();
    println!("🌱 After '{}' iterations the pots with plants sum to '{}' (Part 1)", garden.get_current_iteration(), sum_current_position);

    // Part 1
    garden.run_until_iteration(SECOND_PART_NUMBER_ITERATIONS);
    let sum_current_position = garden.get_iteration_sum_with_plants();
    println!("🌱 After '{}' iterations the pots with plants sum to '{}' (Part 2)", garden.get_current_iteration(), sum_current_position);
}