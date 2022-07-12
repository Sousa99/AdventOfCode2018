use std::collections::HashMap;

// ======================================================== STRUCTS DEFINITIONS ========================================================

pub type GridSerialNumber = u64;
type CoordinateUnit = u64;
type FuelLevel = i64;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate2D {
    x: CoordinateUnit,
    y: CoordinateUnit,
}

struct FuelCell {
    position: Coordinate2D,
    fuel_level: Option<FuelLevel>,
}

pub struct FuelGrid {
    serial_number: GridSerialNumber,
    top_left: Coordinate2D,
    bottom_right: Coordinate2D,
    fuel_grid: HashMap<Coordinate2D, FuelCell>
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Coordinate2D {
    pub fn new(position_x: CoordinateUnit, position_y: CoordinateUnit) -> Coordinate2D {
        Coordinate2D { x: position_x, y: position_y }
    }

    pub fn get_x(&self) -> CoordinateUnit { self.x }
    pub fn get_y(&self) -> CoordinateUnit { self.y }
}

impl FuelCell {
    fn new(position: Coordinate2D) -> FuelCell {
        FuelCell { position: position, fuel_level: None }
    }

    fn get_rack_id(&self) -> FuelLevel { self.position.get_x() as FuelLevel + 10 }
    fn get_begin_power_level(&self) -> FuelLevel { self.get_rack_id() * self.position.get_y() as FuelLevel }
    fn get_increased_power_level(&self, grid_sn: GridSerialNumber) -> FuelLevel { self.get_begin_power_level() + grid_sn as FuelLevel }
    fn get_set_power_level(&self, grid_sn: GridSerialNumber) -> FuelLevel { self.get_increased_power_level(grid_sn) * self.get_rack_id() }
    fn get_hundreds_power_level(&self, grid_sn: GridSerialNumber) -> FuelLevel { (self.get_set_power_level(grid_sn) as i64 / 100) % 10 }
    fn get_subtracted_power_level(&self, grid_sn: GridSerialNumber) -> FuelLevel { self.get_hundreds_power_level(grid_sn) - 5 }

    fn compute_cell_power_level(&mut self, grid_sn: GridSerialNumber) { self.fuel_level = Some(self.get_subtracted_power_level(grid_sn)) }
    fn get_fuel_level(&self) -> Option<FuelLevel> { self.fuel_level }
}

impl FuelGrid {
    pub fn new(top_left_coordinate: Coordinate2D, bottom_right_coordinate: Coordinate2D, serial_number: GridSerialNumber) -> FuelGrid {
        let mut fuel_grid : HashMap<Coordinate2D, FuelCell> = HashMap::new();
        for position_y in top_left_coordinate.get_y()..=bottom_right_coordinate.get_y() {
            for position_x in top_left_coordinate.get_x()..=bottom_right_coordinate.get_x() {
                
                let fuel_position : Coordinate2D = Coordinate2D { x: position_x, y: position_y };
                let fuel_cell : FuelCell = FuelCell::new(fuel_position);
                fuel_grid.insert(fuel_position, fuel_cell);
            }
        }

        FuelGrid { serial_number: serial_number, top_left: top_left_coordinate, bottom_right: bottom_right_coordinate, fuel_grid: fuel_grid }
    }

    pub fn compute_fuel_levels(&mut self) {
        for (_, fuel_cell) in self.fuel_grid.iter_mut() {
            fuel_cell.compute_cell_power_level(self.serial_number);
        }
    }

    fn compute_sum_of_areas_mapping(&self) -> HashMap<Coordinate2D, FuelLevel> {
        let mut sum_areas_mapping : HashMap<Coordinate2D, FuelLevel> = HashMap::new();
        for position_y in (self.top_left.get_y()..=self.bottom_right.get_y()).rev() {
            for position_x in (self.top_left.get_x()..=self.bottom_right.get_x()).rev() {

                let target_position : Coordinate2D = Coordinate2D::new(position_x, position_y);
                let target_fuel_level : FuelLevel = self.fuel_grid.get(&target_position).unwrap().get_fuel_level().unwrap();

                let area_inclusion_right = *sum_areas_mapping.get(&Coordinate2D::new(position_x + 1, position_y)).unwrap_or(&0);
                let area_inclusion_bottom = *sum_areas_mapping.get(&Coordinate2D::new(position_x, position_y + 1)).unwrap_or(&0);
                let area_exclusion_intersection = *sum_areas_mapping.get(&Coordinate2D::new(position_x + 1, position_y + 1)).unwrap_or(&0);

                let sum_fuel_area = target_fuel_level + area_inclusion_right + area_inclusion_bottom - area_exclusion_intersection;
                sum_areas_mapping.insert(target_position, sum_fuel_area);
            }
        }
        
        return sum_areas_mapping;
    }

    fn get_area_sum(&self, area_mapping: &HashMap<Coordinate2D, FuelLevel>, position: Coordinate2D, size_x: CoordinateUnit, size_y: CoordinateUnit) -> FuelLevel {
        let area_total = *area_mapping.get(&position).unwrap();
        let area_exclusion_right = *area_mapping.get(&Coordinate2D::new(position.get_x() + size_x, position.get_y())).unwrap_or(&0);
        let area_exclusion_bottom = *area_mapping.get(&Coordinate2D::new(position.get_x(), position.get_y() + size_y)).unwrap_or(&0);
        let area_exclusion_intersection = *area_mapping.get(&Coordinate2D::new(position.get_x() + size_x, position.get_y() + size_y)).unwrap_or(&0);

        return area_total - area_exclusion_right - area_exclusion_bottom + area_exclusion_intersection;
    }

    fn get_max_fuel_square_size_aux(&self, area_mapping: &HashMap<Coordinate2D, FuelLevel>, size: CoordinateUnit) -> (Coordinate2D, FuelLevel) {

        let mut current_max : Option<(Coordinate2D, FuelLevel)> = None;
        for position_y in self.top_left.get_y()..=(self.bottom_right.get_y() - size as CoordinateUnit + 1) {
            for position_x in self.top_left.get_x()..=(self.bottom_right.get_x() - size as CoordinateUnit + 1) {

                let target_position : Coordinate2D = Coordinate2D::new(position_x, position_y);
                let current_sum : FuelLevel = self.get_area_sum(&area_mapping, target_position, size, size);
                if current_max.is_none() || current_max.unwrap().1 < current_sum { current_max = Some((target_position, current_sum)); }
            }
        }

        return current_max.unwrap();
    }

    pub fn get_max_fuel_square_size(&self, size: CoordinateUnit)-> (Coordinate2D, FuelLevel) {
        let area_mapping = self.compute_sum_of_areas_mapping();
        return self.get_max_fuel_square_size_aux(&area_mapping, size);
    }
    
    pub fn get_max_fuel_square_any(&self) -> (Coordinate2D, CoordinateUnit, FuelLevel) {
        let area_mapping = self.compute_sum_of_areas_mapping();

        let max_size_possible_x : CoordinateUnit = self.bottom_right.get_x() - self.top_left.get_x() + 1;
        let max_size_possible_y : CoordinateUnit = self.bottom_right.get_y() - self.top_left.get_y() + 1;
        let max_size_possible = std::cmp::min(max_size_possible_x, max_size_possible_y);

        return (1..=max_size_possible)
            .map(|size| (size, self.get_max_fuel_square_size_aux(&area_mapping, size)))
            .map(|(size, (coordinate, fuel))| (coordinate, size, fuel))
            .max_by_key(|(_, _, fuel)| *fuel)
            .unwrap();
    }


    pub fn _print_formatted(&self) -> String {

        let mut final_string : String = format!("⚡ Grid for a serial number of '{}':\n", self.serial_number);
        for position_y in self.top_left.get_y()..=self.bottom_right.get_y() {
            for position_x in self.top_left.get_x()..=self.bottom_right.get_x() {
                
                let check_position : Coordinate2D = Coordinate2D::new(position_x, position_y);
                let fuel_level_option : Option<FuelLevel> = self.fuel_grid.get(&check_position).unwrap().get_fuel_level();
                match fuel_level_option {
                    Some(fuel_level) if fuel_level >= 0 => final_string.push_str(&format!("+{} ", fuel_level)),
                    Some(fuel_level) => final_string.push_str(&format!("{} ", fuel_level)),
                    None => final_string.push_str("?? "),
                }
            }

            final_string.push('\n');
        }

        return final_string;
    }
}