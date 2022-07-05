use std::collections::{HashMap, HashSet};

// ======================================================= CONSTANTS DEFINITIONS =======================================================

// ======================================================== STRUCTS DEFINITIONS ========================================================

#[derive(PartialEq, Clone)]
enum PolymerUnitPolarity { Lowercase, Uppercase }
type PolymerUnitType = char;

#[derive(Clone, Debug)]
pub struct PolymerUnit(char);

#[derive(Clone)]
pub struct Polymer {
    initial_state: Vec<PolymerUnit>,
    states: Vec<Vec<PolymerUnit>>
}
pub struct ProblematicPolymer {
    initial_state: Vec<PolymerUnit>,
    polymer_types: Vec<PolymerUnitType>,
    states: HashMap<PolymerUnitType, (Vec<PolymerUnit>, usize)>
}

// ======================================================== STRUCTS IMPLEMENTATIONS ========================================================

impl PolymerUnit {
    pub fn get_char(&self) -> char { return self.0; }
    fn get_polarity(&self) -> PolymerUnitPolarity {
        if self.get_char().is_lowercase() { return PolymerUnitPolarity::Lowercase; }
        else if self.get_char().is_uppercase() { return PolymerUnitPolarity::Uppercase; }
        else { panic!("🚨 Polarity for '{}' could not be established!", self.get_char()) }
    }
    fn get_type(&self) -> PolymerUnitType { self.get_char().to_uppercase().nth(0).unwrap() }
}

impl Polymer {
    pub fn new(initial_state_chars: &Vec<char>) -> Polymer {
        Polymer {
            initial_state: initial_state_chars.into_iter()
                .map(|char| PolymerUnit(*char))
                .collect(),
            states: Vec::new()
        }
    }

    pub fn new_from_polymers(initial_state: Vec<PolymerUnit>) -> Polymer {
        Polymer { initial_state: initial_state, states: Vec::new() }
    }

    pub fn run_iteration(&mut self) -> bool {

        let start_point : &Vec<PolymerUnit> = self.states.last()
            .unwrap_or(&self.initial_state);
        let verification_points = start_point.iter().zip(start_point.iter().skip(1));

        let mut new_polymer : Vec<PolymerUnit> = Vec::new();

        let mut some_changed : bool = false;
        let mut last_changed : bool = false;

        for (polymer_a, polymer_b) in verification_points {
            let polymer_a_type = polymer_a.get_type();
            let polymer_b_type = polymer_b.get_type();

            if last_changed { last_changed = false; }
            else if polymer_a_type != polymer_b_type || polymer_a.get_polarity() == polymer_b.get_polarity() {
                new_polymer.push(polymer_a.clone());
                last_changed = false;
            } else {
                some_changed = true;
                last_changed = true;
            }
        }

        if !last_changed { new_polymer.push(start_point.last().unwrap().clone()); }
        if some_changed { self.states.push(new_polymer); }
        return some_changed;
    }

    pub fn get_last_iteration(&self) -> &Vec<PolymerUnit> { self.states.last().unwrap_or(&self.initial_state) }
}

impl ProblematicPolymer {
    pub fn new(initial_state_chars: &Vec<char>) -> ProblematicPolymer {

        let initial_state : Vec<PolymerUnit> = initial_state_chars.into_iter()
            .map(|char| PolymerUnit(*char))
            .collect();
        let types : Vec<PolymerUnitType> = initial_state.iter()
            .map(|unit| unit.get_type())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        ProblematicPolymer { 
            initial_state: initial_state,
            polymer_types: types,
            states: HashMap::new(),
        }
    }

    pub fn run_polymers(&mut self) {

        for polymer_type_remove in self.polymer_types.iter() {
            let new_initial_state : Vec<PolymerUnit> = self.initial_state.iter()
                .filter(|&unit| unit.get_type() != *polymer_type_remove)
                .map(|unit| unit.clone())
                .collect();

            let mut new_polymer : Polymer = Polymer::new_from_polymers(new_initial_state);
            let mut some_changed : bool = true;
            while some_changed { some_changed = new_polymer.run_iteration(); }

            let final_polymer = new_polymer.get_last_iteration();
            let final_polymer_size = final_polymer.len();
            
            self.states.insert(*polymer_type_remove, (final_polymer.clone(), final_polymer_size));
        }
    }

    pub fn get_less_problematic(&self) -> Option<(PolymerUnitType, Vec<PolymerUnit>, usize)> {
        self.states.iter()
            .map(|(removed_unit, (final_polymer, polymer_size))| (removed_unit, final_polymer, polymer_size))
            .min_by_key(|(_, _, &polymer_size)| polymer_size)
            .map(|(removed_unit, final_polymer, &polymer_size)| (*removed_unit, final_polymer.clone(), polymer_size))
    }
}