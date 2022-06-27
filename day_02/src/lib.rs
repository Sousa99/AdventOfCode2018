use std::collections::HashMap;

// ======================================================== STRUCTS DEFINITIONS ========================================================

struct WarehouseBox {
    id: String,
}

pub struct Warehouse {
    boxes: Vec<WarehouseBox>,
}

// ======================================================== STRUCTS IMPLEMENTATIONS ========================================================

impl WarehouseBox {
    fn new(id: String) -> WarehouseBox { WarehouseBox { id: id } }

    fn get_count_map(&self) -> HashMap<char, i64> {
        let mut hash_map : HashMap<char, i64> = HashMap::new();
        for characther in self.id.chars() {
            if !hash_map.contains_key(&characther) { hash_map.insert(characther, 0); }
            let current_value = hash_map.get_mut(&characther).unwrap();
            *current_value = *current_value + 1;
        }

        return hash_map;
    }

    fn check_any_exact(&self, exact_number: i64) -> bool {
        let hash_map = self.get_count_map();
        for (_, hash_value) in hash_map.into_iter() {
            if hash_value == exact_number {
                return true;
            }
        }

        return false;
    }

    fn get_matched_difference(&self, other_box: &WarehouseBox) -> (i64, String) {
        let this_id = self.id.chars();
        let other_id = other_box.id.chars();

        let mut current_count : i64 = 0;
        let mut matched : Vec<char> = Vec::new();
        for (this_characther, other_characther) in std::iter::zip(this_id, other_id) {
            if this_characther == other_characther { matched.push(this_characther); }
            else { current_count = current_count + 1; }
        }

        return (current_count, matched.into_iter().collect());
    }
}

impl Warehouse {
    pub fn new(ids: Vec<String>) -> Warehouse {
        Warehouse {
            boxes: ids.into_iter()
                .map(|id| WarehouseBox::new(id))
                .collect(),
        }
    }

    pub fn get_number_verify_exact(&self, exact_number: i64) -> i64 {
        let mut current_counter : i64 = 0;
        for warehouse_box in self.boxes.iter() {
            if warehouse_box.check_any_exact(exact_number) {
                current_counter = current_counter + 1;
            }
        }

        return current_counter;
    }

    pub fn get_matched_boxes(&self, non_matched_count: i64) -> Option<String> {
        for (warehouse_box_1_index, warehouse_box_1) in self.boxes.iter().enumerate() {
            let slice = &self.boxes[(warehouse_box_1_index + 1)..];
            for warehouse_box_2 in slice.iter() {

                let (difference_count, matched) = warehouse_box_1.get_matched_difference(warehouse_box_2);
                if non_matched_count == difference_count { return Some(matched); }
            }
        }

        return None;
    }
}