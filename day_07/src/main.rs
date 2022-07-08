mod read;
mod lib;

use lib::{DependenceSolver, DependenceSolverMultiple};
fn main() {

    let dependents_strings = read::read_lines("input.txt".to_owned());
    
    // Part 1
    let solver = DependenceSolver::new(&dependents_strings);
    let correct_sequence = solver.solve_best_order();
    println!("\r🛠️  Best task order possible: '{}' (Part 1)", correct_sequence.iter().collect::<String>());
    
    // Part 2
    let number_of_workers : usize = 5;
    let base_delay : i64 = 60;
    let multi_solver = DependenceSolverMultiple::new(&dependents_strings, number_of_workers, base_delay);
    let multi_solution = multi_solver.solve_best_order();
    println!("\r🛠️  Best task order possible ('{}' workers, '{}' base delay): '{}' in '{}' timesteps (Part 2)",
        number_of_workers, base_delay, multi_solution.0.iter().collect::<String>(), multi_solution.1);
}