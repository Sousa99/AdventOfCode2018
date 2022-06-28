

// ======================================================== STRUCTS DEFINITIONS ========================================================

use std::collections::{HashMap, HashSet};

pub struct Claim {
    id: i64,
    begin: (i64, i64),
    size: (i64, i64)
}

pub struct FabricManager {
    claims: HashMap<i64, Claim>,
    current_limits: Option<((i64, i64), (i64, i64))>,
    current_map: HashMap<(i64, i64), Vec<i64>>
}

// ======================================================== STRUCTS IMPLEMENTATIONS ========================================================

impl Claim {
    pub fn new(line: String) -> Claim {
        let splitted : Vec<String> = line.replace("#", "").replace(" @ ", " ")
            .replace(",", " ").replace(": ", " ").replace("x", " ")
            .split(" ").map(|s| s.to_owned()).collect();

        Claim {
            id: splitted.get(0).unwrap().parse::<i64>().unwrap(),
            begin: (splitted.get(1).unwrap().parse::<i64>().unwrap(), splitted.get(2).unwrap().parse::<i64>().unwrap()),
            size: (splitted.get(3).unwrap().parse::<i64>().unwrap(), splitted.get(4).unwrap().parse::<i64>().unwrap())
        }
    }

    fn get_id(&self) -> i64 { self.id }
    fn get_claimed_positions(&self) -> Vec<(i64, i64)> {
        let mut claimed_positions : Vec<(i64, i64)> = Vec::new();
        for x_var in 0..self.size.0 {
            for y_var in 0..self.size.1 {
                claimed_positions.push((self.begin.0 + x_var, self.begin.1 + y_var))
            }
        }

        return claimed_positions;
    }
}

impl FabricManager {
    pub fn new(claims: Vec<Claim>) -> FabricManager {
        FabricManager {
            claims: claims.into_iter().map(|claim| (claim.get_id(), claim)).collect(),
            current_limits: None,
            current_map: HashMap::new(),
        }
    }

    pub fn develop_mapping(&mut self) {
        for (claim_id, claim) in self.claims.iter() {
            for claimed_position in claim.get_claimed_positions() {

                // Update Limits
                if self.current_limits.is_none() { self.current_limits = Some((claimed_position, claimed_position)) }
                
                let mut limits = self.current_limits.unwrap(); 
                if limits.0.0 > claimed_position.0 { limits.0.0 = claimed_position.0; }
                if limits.0.1 > claimed_position.1 { limits.0.1 = claimed_position.1; }
                if limits.1.0 < claimed_position.0 { limits.1.0 = claimed_position.0; }
                if limits.1.1 < claimed_position.1 { limits.1.1 = claimed_position.1; }
                self.current_limits = Some(limits);
                
                // Add results to mapping
                if !self.current_map.contains_key(&claimed_position) { self.current_map.insert(claimed_position, Vec::new()); }
                self.current_map.get_mut(&claimed_position).unwrap().push(*claim_id);
            }
        }
    }

    pub fn check_mapping_position_equal_or_higher(&self, value_to_check: usize) -> i64 {
        let mut current_counter : i64 = 0;
        for (_, claims) in self.current_map.iter() {
            if claims.len() >= value_to_check {
                current_counter = current_counter + 1;
            }
        }

        return current_counter;
    }

    pub fn get_claim_without_conflicts(&self) -> Option<i64> {
        let mut claims_with_conflicts : HashSet<i64> = HashSet::new();
        for (_, claims) in self.current_map.iter() {
            if claims.len() >= 2 {
                for claim in claims.iter() {
                    claims_with_conflicts.insert(*claim);
                }
            }
        }

        for (claim_id, _) in self.claims.iter() {
            if !claims_with_conflicts.contains(claim_id) {
                return Some(*claim_id);
            }
        }
        
        return None;
    }
}