

// ======================================================== STRUCTS DEFINITIONS ========================================================

type RecipeValue = i64;

#[derive(Clone, Copy)]
pub struct Recipe { value: RecipeValue }
pub struct RecipeManager { itearation: usize, elves: Vec<usize>, recipes: Vec<Recipe>, improvement_size: usize }

// ======================================================== AUXILIARY FUNCTIONS ========================================================



// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Recipe {
    pub fn new(recipe_value: RecipeValue) -> Recipe {
        Recipe { value: recipe_value }
    }

    fn get_value(&self) -> RecipeValue { self.value }
}

impl RecipeManager {
    pub fn new(number_elves: usize, recipes: Vec<Recipe>, improvement_size: usize) -> RecipeManager {
        RecipeManager {
            itearation: 0,
            elves: (0..number_elves).into_iter().collect(),
            recipes: recipes,
            improvement_size: improvement_size,
        }
    }

    pub fn get_iteration(&self) -> usize { self.itearation }

    pub fn estimate_improvement(&self, after_value: usize) -> Option<String> {
        if self.recipes.len() < after_value + self.improvement_size { return None }
        return Some((0..self.improvement_size).into_iter().rev()
            .map(|index_offset| self.recipes.get(after_value + index_offset).unwrap())
            .map(|recipe| recipe.get_value().to_string())
            .rev()
            .collect::<Vec<String>>()
            .join(""));
    }

    pub fn compare_last_recipe_match(&self, recipe_value: &str) -> Option<usize> {
        let recipe_len = recipe_value.len();

        if self.recipes.len() < recipe_len { return None; }
        let final_recipe_first : String = (0..recipe_len).into_iter()
            .map(|index| self.recipes.len() - 1 - index)
            .map(|index| self.recipes.get(index).unwrap())
            .map(|recipe| recipe.get_value().to_string())
            .rev()
            .collect::<Vec<String>>()
            .join("");
        if &final_recipe_first == recipe_value { return Some(self.recipes.len() - recipe_len); }
        
        if self.recipes.len() < recipe_len + 1 { return None; }
        let final_recipe_second : String = (1..(recipe_len + 1)).into_iter()
            .map(|index| self.recipes.len() - 1 - index)
            .map(|index| self.recipes.get(index).unwrap())
            .map(|recipe| recipe.get_value().to_string())
            .rev()
            .collect::<Vec<String>>()
            .join("");
        if &final_recipe_second == recipe_value { return Some(self.recipes.len() - recipe_len - 1); }

        return None;
    }

    pub fn run_iteration(&mut self) {

        self.itearation = self.itearation + 1;

        let current_number_recipes : usize = self.recipes.len();
        let elves_values : Vec<RecipeValue> = self.elves.iter()
            .map(|elf_index| elf_index % current_number_recipes)
            .map(|elf_index| self.recipes.get(elf_index).unwrap())
            .map(|recipe| recipe.get_value())
            .collect();

        let sum_elves_values : RecipeValue = elves_values.iter().sum();
        let number_digits : usize = (sum_elves_values as f64).log10().floor() as usize + 1;
        let new_values : Vec<RecipeValue> = (0..number_digits).into_iter()
            .map(|digit| (sum_elves_values / ((10 as RecipeValue).pow(digit as u32))) % 10)
            .rev()
            .collect();

        for new_value in new_values.into_iter() { self.recipes.push(Recipe::new(new_value)); }
        let current_number_recipes : usize = self.recipes.len();
        self.elves = self.elves.iter().zip(elves_values.into_iter())
            .map(|(&elf_index, elf_value)| (elf_index + 1 + elf_value as usize) % current_number_recipes)
            .collect();
    }

    pub fn _print_formatted(&self) -> String {

        let elf_1_index : usize = *self.elves.get(0).unwrap();
        let elf_2_index : usize = *self.elves.get(1).unwrap();

        let mut final_string : String = format!("☕ On iteration '{}': ", self.itearation);
        let recipe_values_string : String = self.recipes.iter().enumerate()
            .map(|(index, recipe)| {
                if index == elf_1_index { format!("({})", recipe.get_value()) }
                else if index == elf_2_index { format!("[{}]", recipe.get_value()) }
                else { format!(" {} ", recipe.get_value()) }})
            .collect::<Vec<String>>()
            .join(" ");

        final_string.push_str(&recipe_values_string);
        return final_string;
    }
}