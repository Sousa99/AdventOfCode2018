use std::collections::HashMap;

// ======================================================== STRUCTS DEFINITIONS ========================================================

type PotPosition = i64;

#[derive(PartialEq, Eq, Clone, Copy)]
enum PotState { WithPlant, WithoutPlant }

#[derive(Clone, Copy)]
pub struct Pot {
    state: PotState,
}

pub struct Rule {
    requesits: Vec<(PotPosition, PotState)>,
    result: PotState,
}

pub struct Garden {
    current_iteration: usize,
    pots: HashMap<PotPosition, Pot>,
    rules: Vec<Rule>
}

// ======================================================== AUXILIARY FUNCTIONS== ======================================================

fn convert_char_to_pot_state(char_state: char) -> PotState {
    match char_state {
        '#' => PotState::WithPlant,
        '.' => PotState::WithoutPlant,
        _   => panic!("🚨 Pot state '{}' not recognized!", char_state)
    }
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Pot {
    pub fn new(pot_state: char) -> Pot {
        let pot_state : PotState = match pot_state {
            '#' => PotState::WithPlant,
            '.' => PotState::WithoutPlant,
            _   => panic!("🚨 Pot state '{}' not recognized!", pot_state)};
        Pot { state: pot_state }
    }

    fn new_with_state(pot_state: PotState) -> Pot {
        Pot { state: pot_state }
    }

    fn _format_pot(&self) -> char {
        match self.state {
            PotState::WithPlant => '#',
            PotState::WithoutPlant => '.',
        }
    }

    fn matches_state(&self, state: PotState) -> bool { self.state == state }
}

impl Rule {
    pub fn new(requisit_chars: Vec<char>, result_char: char) -> Rule {
        let origin : PotPosition = requisit_chars.len() as PotPosition / 2;
        let requesits : Vec<(PotPosition, PotState)> = requisit_chars.into_iter().enumerate()
            .map(|(index, requisit_char)| (index as PotPosition - origin, convert_char_to_pot_state(requisit_char)))
            .collect();
        let result : PotState = convert_char_to_pot_state(result_char);
        return Rule { requesits: requesits, result: result };
    }
}

impl Garden {
    pub fn new(initial_state: Vec<Pot>, rules: Vec<Rule>) -> Garden {
        Garden {
            current_iteration: 0,
            pots: initial_state.into_iter().enumerate()
                .map(|(pot_index, pot)| (pot_index as PotPosition, pot))
                .collect(),
            rules: rules,
        }
    }

    pub fn get_current_iteration(&self) -> usize { self.current_iteration }

    fn get_first_check_position(&self) -> PotPosition {
        let max_rule_size : PotPosition = self.rules.iter()
            .map(|rule| rule.requesits.len())
            .max().unwrap() as PotPosition;
        return self.pots.iter()
            .filter(|(_, pot)| pot.matches_state(PotState::WithPlant))
            .map(|(pot_position, _)| pot_position)
            .min().unwrap() - (max_rule_size / 2);
    }

    fn get_last_check_position(&self) -> PotPosition {
        let max_rule_size : PotPosition = self.rules.iter()
            .map(|rule| rule.requesits.len())
            .max().unwrap() as PotPosition;
        return self.pots.iter()
            .filter(|(_, pot)| pot.matches_state(PotState::WithPlant))
            .map(|(pot_position, _)| pot_position)
            .max().unwrap() + (max_rule_size / 2);
    }

    fn translate_pots(&mut self, translate_first_to: PotPosition) {
        let first_position : PotPosition = self.get_first_check_position();
        let translation : PotPosition = translate_first_to - first_position;
        let new_pots : HashMap<PotPosition, Pot> = self.pots.iter()
            .map(|(&pot_position, &pot)| (pot_position + translation, pot.clone()))
            .collect();

        self.pots = new_pots;
    }

    fn run_iteration(&mut self) {

        // Get Limits for Rule Testing
        let first_position : PotPosition = self.get_first_check_position();
        let last_position : PotPosition = self.get_last_check_position();
        
        // Iterate and check for rules
        let mut new_set_of_pots : HashMap<PotPosition, Pot> = HashMap::new();
        for current_position in first_position..=last_position {
            let mut rule_matched : Option<&Rule> = None;
            for rule in self.rules.iter() {
                let matched : bool = rule.requesits.iter()
                    .map(|&(position_variation, pot_requisit_state)| (self.pots.get(&(current_position + position_variation)), pot_requisit_state) )
                    .all(|(pot_on_position_option, pot_requisit_state)| {
                        match pot_on_position_option {
                            Some(pot) => pot.matches_state(pot_requisit_state),
                            None if pot_requisit_state == PotState::WithoutPlant => true,
                            None => false,
                        }});
                if matched { rule_matched = Some(rule); }
            }

            let state_to_be_associated : PotState = match rule_matched {
                Some(rule) => rule.result,
                None => PotState::WithoutPlant
            };

            if state_to_be_associated != PotState::WithoutPlant {
                new_set_of_pots.insert(current_position, Pot::new_with_state(state_to_be_associated));
            }
        }

        // Update Garden
        self.current_iteration = self.current_iteration + 1;
        self.pots = new_set_of_pots;

    }

    pub fn run_until_iteration(&mut self, iteration: usize) {

        let mut already_verified : HashMap<String, (usize, PotPosition)> = HashMap::new();
        while self.get_current_iteration() != iteration {

            // Run iteration
            self.run_iteration();
            // Get current iteration code
            let current_code : String = self.get_iteration_code();

            // Deal with already verified
            let matched_iteration_option = already_verified.get(&current_code);
            if matched_iteration_option.is_some() {
                let matched_iteration = *matched_iteration_option.unwrap();

                // Get Iteration Jump
                let jump = self.current_iteration - matched_iteration.0;
                let number_of_iterations_missing = iteration - self.current_iteration;
                let number_of_jumps = number_of_iterations_missing / jump;
                // Get Translation Jump
                let first_position = self.get_first_check_position();
                let translation = first_position - matched_iteration.1;

                self.current_iteration = self.current_iteration + number_of_jumps * jump;
                self.translate_pots(first_position + number_of_jumps as PotPosition * translation);
            } else { already_verified.insert(current_code, (self.current_iteration, self.get_first_check_position())); }
        }
    }

    pub fn get_iteration_sum_with_plants(&self) -> i64 {
        return self.pots.iter()
            .filter(|&(_, pot)| pot.matches_state(PotState::WithPlant))
            .map(|(&pot_position, _)| pot_position)
            .sum();
    }

    fn get_iteration_code(&self) -> String {
        let first_position : PotPosition = self.get_first_check_position();
        let last_position : PotPosition = self.get_last_check_position();
        return (first_position..=last_position)
            .map(|pot_position| {
                match self.pots.get(&pot_position) {
                    Some(pot) => pot._format_pot(),
                    None => '.',
                }})
            .collect::<String>();
    }
        
    pub fn _print_iteration(&self) -> String {
        let first_position : PotPosition = self.get_first_check_position();
        return format!("🌱 Iteration [{}]: {} (starting on '{}')", self.current_iteration, self.get_iteration_code(), first_position);
    }
}