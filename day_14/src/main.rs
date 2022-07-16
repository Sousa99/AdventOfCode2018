mod read;
mod lib;

use lib::{Recipe, RecipeManager};

fn main() {

    let lines_read : Vec<i64> = read::read_int_lines("input.txt".to_owned());
    let input_number : usize = *lines_read.get(0).unwrap() as usize;
    let input_number_string : String = input_number.to_string();

    // Constants definition
    let number_of_elfes : usize = 2;
    let size_recipe_improvement : usize = 10;
    let original_recipes : Vec<i64> = vec![3, 7];

    // Create recipes and recipe manager
    let recipes : Vec<Recipe> = original_recipes.into_iter()
        .map(|recipe_value| Recipe::new(recipe_value))
        .collect();
        
    // Part 1
    let mut recipe_manager = RecipeManager::new(number_of_elfes, recipes.clone(), size_recipe_improvement);
    let mut final_recipe_estimation : Option<String> = None;
    //println!("{}", recipe_manager._print_formatted());
    while final_recipe_estimation.is_none() {
        recipe_manager.run_iteration();
        //println!("{}", recipe_manager._print_formatted());
        final_recipe_estimation = recipe_manager.estimate_improvement(input_number);
    }
    let current_iteration : usize = recipe_manager.get_iteration();
    println!("☕ After '{}' iterations, the final recipe estimated immediately after '{}' is '{}' (Part 1)", current_iteration, input_number, final_recipe_estimation.unwrap());
    
    // Part 2
    let mut recipe_manager = RecipeManager::new(number_of_elfes, recipes, size_recipe_improvement);
    let mut scores_to_the_left : Option<usize> = None;
    //println!("{}", recipe_manager._print_formatted());
    while scores_to_the_left.is_none() {
        recipe_manager.run_iteration();
        //println!("{}", recipe_manager._print_formatted());
        scores_to_the_left = recipe_manager.compare_last_recipe_match(&input_number_string);
    }
    let current_iteration : usize = recipe_manager.get_iteration();
    println!("☕ After '{}' iterations, the recipe is matched '{}' with '{}' scores to the left (Part 1)", current_iteration, input_number_string, scores_to_the_left.unwrap());
}