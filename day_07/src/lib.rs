use regex::Regex;
use std::collections::{HashSet, HashMap};

// ======================================================== STRUCTS DEFINITIONS ========================================================

type NodeID = char;

pub struct DependenceSolver {
    nodes: HashSet<NodeID>,
    dependences: HashMap<NodeID, HashSet<NodeID>>
}

type TimeStep = i64;

pub struct DependenceSolverMultiple {
    nodes: HashSet<NodeID>,
    number_workers: usize,
    time_delay: TimeStep,
    dependences: HashMap<NodeID, HashSet<NodeID>>
} 

// ======================================================== AUXILIARY FUNCTIONS ========================================================

fn get_nodes_and_dependences(dependents_strings: &Vec<String>) -> (HashSet<NodeID>, HashMap<NodeID, HashSet<NodeID>>) {

    let dependency_regex : Regex = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.").unwrap();
    let mut nodes : HashSet<NodeID> = HashSet::new();
    let mut dependences : HashMap<NodeID, HashSet<NodeID>> = HashMap::new();

    for dependent_string in dependents_strings.into_iter() {

        let capture_groups_regex = dependency_regex.captures(dependent_string).unwrap();
        let requirement_id : NodeID = capture_groups_regex.get(1).unwrap().as_str()
            .chars().nth(0).unwrap();
        let for_id : NodeID = capture_groups_regex.get(2).unwrap().as_str()
            .chars().nth(0).unwrap();

        nodes.insert(requirement_id);
        nodes.insert(for_id);
        if !dependences.contains_key(&requirement_id) { dependences.insert(requirement_id, HashSet::new()); }
        if !dependences.contains_key(&for_id) { dependences.insert(for_id, HashSet::new()); }

        let dependencies_for = dependences.get_mut(&for_id).unwrap();
        dependencies_for.insert(requirement_id);
    }

    return (nodes, dependences);
}

fn convert_to_ascii(node: &NodeID) -> u64 {
    return *node as u64;
}

fn get_node_delay(node: &NodeID) -> TimeStep {

    let first_delay : TimeStep = convert_to_ascii(&'A') as TimeStep;
    let node_delay_unbased : TimeStep = convert_to_ascii(node) as TimeStep;
    return node_delay_unbased - first_delay + 1; 
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl DependenceSolver {
    pub fn new(dependents_strings: &Vec<String>) -> DependenceSolver {

        let (nodes, dependences) = get_nodes_and_dependences(dependents_strings);
        DependenceSolver { nodes: nodes, dependences: dependences }
    }

    pub fn solve_best_order(&self) -> Vec<NodeID> {

        let mut correct_sequence : Vec<NodeID> = Vec::new();
        while self.nodes.len() > correct_sequence.len() {

            // Find possible choices
            let valid_choices : Vec<NodeID> = self.nodes.iter()
                .filter(|node| !correct_sequence.contains(node))
                .map(|node| (node, self.dependences.get(node).unwrap().clone()))
                .map(|(node, mut requirements)| {
                    for node_completed in correct_sequence.iter() { requirements.remove(node_completed); }
                    return (node, requirements) })
                .filter(|(_, requirements)| requirements.len() == 0)
                .map(|(node, _)| *node)
                .collect();

            // Get choice
            if valid_choices.len() == 0 { panic!("🚨 No possible valid choice for resolution!") }
            let choice = valid_choices.iter().min().unwrap();
            correct_sequence.push(*choice);
        }

        return correct_sequence;
    }
}

impl DependenceSolverMultiple {
    pub fn new(dependents_strings: &Vec<String>, number_workers: usize, time_delay: TimeStep) -> DependenceSolverMultiple {

        let (nodes, dependences) = get_nodes_and_dependences(dependents_strings);
        DependenceSolverMultiple { nodes: nodes, number_workers: number_workers, time_delay: time_delay, dependences: dependences }
    }

    pub fn solve_best_order(&self) -> (Vec<NodeID>, TimeStep) {

        let mut current_timestep : TimeStep = 0;
        let mut correct_sequence : Vec<NodeID> = Vec::new();
        let mut on_process_nodes : HashSet<NodeID> = HashSet::new();
        let mut workers_schedule : HashMap<usize, Vec<(NodeID, TimeStep)>> = (0..self.number_workers).map(|worker| (worker, Vec::new())).collect();
        while self.nodes.len() > correct_sequence.len() {

            // Mark nodes completed
            for (_, worker_schedule) in workers_schedule.iter() {
                if worker_schedule.len() == 0 { continue; }
                let last_item = worker_schedule.last().unwrap();
                if last_item.1 == current_timestep { correct_sequence.push(last_item.0) }
            }
            
            // Find available workers
            let mut available_workers : Vec<usize> = workers_schedule.iter()
                .map(|(worker, schedule)| (worker, schedule.last()))
                .filter(|(_, schedule)| {
                    match schedule {
                        Some(schedule) => schedule.1 <= current_timestep,
                        None => true,
                    }})
                .map(|(worker, _)| *worker).collect();
                
            // Find possible choices
            let mut valid_choices : Vec<NodeID> = self.nodes.iter()
                .filter(|node| !on_process_nodes.contains(node))
                .map(|node| (node, self.dependences.get(node).unwrap().clone()))
                .map(|(node, mut requirements)| {
                    for node_completed in correct_sequence.iter() { requirements.remove(node_completed); }
                    return (node, requirements) })
                .filter(|(_, requirements)| requirements.len() == 0)
                .map(|(node, _)| *node)
                .collect();

            // Match available choices with available workers
            available_workers.sort();
            valid_choices.sort();
            for (worker, choice) in available_workers.into_iter().zip(valid_choices.into_iter()) {
                let timestep_delay = self.time_delay + get_node_delay(&choice);
                let worker_schedule = workers_schedule.get_mut(&worker).unwrap();
                worker_schedule.push((choice, current_timestep + timestep_delay));
                on_process_nodes.insert(choice);
            }

            // Update TimeStep
            current_timestep = current_timestep + 1;
        }

        return (correct_sequence, current_timestep - 1);
    }
}