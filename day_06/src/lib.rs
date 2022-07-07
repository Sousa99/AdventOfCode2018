use std::collections::{HashMap, HashSet};

// ======================================================== STRUCTS DEFINITIONS ========================================================

pub type CoordinateUnit = i64;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct CoordinatePoint {
    x: CoordinateUnit,
    y: CoordinateUnit
}

pub struct Map {
    points: Vec<CoordinatePoint>,
    top_left: CoordinatePoint,
    bottom_right: CoordinatePoint,
    closest_mapping: HashMap<CoordinatePoint, HashSet<usize>>,
    full_mapping: HashMap<CoordinatePoint, HashMap<usize, CoordinateUnit>>
}

// ======================================================= AUXILIARY FUNCTIONS =======================================================

fn manhantan_distance(point_1: &CoordinatePoint, point_2: &CoordinatePoint) -> CoordinateUnit {
    let x_difference : CoordinateUnit = point_1.get_x() - point_2.get_x();
    let y_difference : CoordinateUnit = point_1.get_y() - point_2.get_y();
    return x_difference.abs() + y_difference.abs();
}

// ======================================================== STRUCTS IMPLEMENTATIONS ========================================================

impl CoordinatePoint {
    pub fn new(point_x: CoordinateUnit, point_y: CoordinateUnit) -> CoordinatePoint {
        CoordinatePoint { x: point_x, y: point_y }
    }

    pub fn get_x(&self) -> CoordinateUnit { self.x }
    pub fn get_y(&self) -> CoordinateUnit { self.y }
}

impl Map {
    pub fn new(points: Vec<CoordinatePoint>) -> Map {

        let mut top_left_x : Option<CoordinateUnit> = None;
        let mut top_left_y : Option<CoordinateUnit> = None;
        let mut bottom_right_x : Option<CoordinateUnit> = None;
        let mut bottom_right_y : Option<CoordinateUnit> = None;

        for point in points.iter() {
            if top_left_x.is_none() || top_left_x.unwrap() > point.get_x() { top_left_x = Some(point.get_x()) }
            if top_left_y.is_none() || top_left_y.unwrap() > point.get_y() { top_left_y = Some(point.get_y()) }
            if bottom_right_x.is_none() || bottom_right_x.unwrap() < point.get_x() { bottom_right_x = Some(point.get_x()) }
            if bottom_right_y.is_none() || bottom_right_y.unwrap() < point.get_y() { bottom_right_y = Some(point.get_y()) }
        }

        Map {
            points: points,
            top_left: CoordinatePoint::new(top_left_x.unwrap() - 5, top_left_y.unwrap() - 5),
            bottom_right: CoordinatePoint::new(bottom_right_x.unwrap() + 5, bottom_right_y.unwrap() + 5),
            closest_mapping: HashMap::new(),
            full_mapping: HashMap::new()
        }
    }

    pub fn compute_mapping(&mut self) {

        for verify_point_x in self.top_left.get_x()..=self.bottom_right.get_x() {
            for verify_point_y in self.top_left.get_y()..=self.bottom_right.get_y() {

                let verify_point : CoordinatePoint = CoordinatePoint::new(verify_point_x, verify_point_y);

                let mut current_min_distance : Option<CoordinateUnit> = None;
                let mut current_points : HashSet<usize> = HashSet::new();
                let mut current_distances : HashMap<usize, CoordinateUnit> = HashMap::new();
                
                for (index_point, coordinate_point) in self.points.iter().enumerate() {
                    
                    let distance = manhantan_distance(coordinate_point, &verify_point);
                    current_distances.insert(index_point, distance);

                    if current_min_distance.is_none() || current_min_distance.unwrap() > distance {
                        current_min_distance = Some(distance);
                        current_points = HashSet::new();
                        current_points.insert(index_point);
                    } else if current_min_distance.is_some() && current_min_distance.unwrap() == distance {
                        current_points.insert(index_point);
                    } 
                }

                self.closest_mapping.insert(verify_point, current_points);
                self.full_mapping.insert(verify_point, current_distances);
            }
        }

    }

    fn get_limitted_indexes(&self) -> HashSet<usize> {

        let mut current_indexes : HashSet<usize> = self.points.iter().enumerate().map(|(index, _)| index).collect();
        for check_point_x in self.top_left.get_x()..=self.bottom_right.get_x() {
            for check_point_y in vec![self.top_left.get_y(), self.bottom_right.get_y()] {
                let check_point : CoordinatePoint = CoordinatePoint::new(check_point_x, check_point_y);
                let associated_indexes = self.closest_mapping.get(&check_point).unwrap();
                if associated_indexes.len() == 1 {
                    let associated_index = associated_indexes.iter().next().unwrap();
                    current_indexes.remove(associated_index);
                }
            }
        }
        for check_point_x in vec![self.top_left.get_x(), self.bottom_right.get_x()] {
            for check_point_y in self.top_left.get_y()..=self.bottom_right.get_y() {
                let check_point : CoordinatePoint = CoordinatePoint::new(check_point_x, check_point_y);
                let associated_indexes = self.closest_mapping.get(&check_point).unwrap();
                if associated_indexes.len() == 1 {
                    let associated_index = associated_indexes.iter().next().unwrap();
                    current_indexes.remove(associated_index);
                }
            }
        }

        return current_indexes;
    }

    pub fn get_area_for_limitted(&self) -> HashMap<usize, usize> {

        let limitted_indexes = self.get_limitted_indexes();
        let mut mapping_areas : HashMap<usize, usize> = limitted_indexes.iter()
            .map(|&index| (index, 0)).collect();

        for check_point_x in self.top_left.get_x()..=self.bottom_right.get_x() {
            for check_point_y in self.top_left.get_y()..=self.bottom_right.get_y() {

                let check_point : CoordinatePoint = CoordinatePoint::new(check_point_x, check_point_y);
                let associated_indexes = self.closest_mapping.get(&check_point).unwrap();
                if associated_indexes.len() == 1 {
                    let associated_index = associated_indexes.iter().next().unwrap();
                    if limitted_indexes.contains(associated_index) {
                        let current_area = mapping_areas.get_mut(associated_index).unwrap();
                        *current_area = *current_area + 1;
                    }
                }

            }
        }
        
        return mapping_areas;
    }

    pub fn get_points_with_sum_less(&self, threshold: CoordinateUnit) -> HashSet<CoordinatePoint> {

        let mut valid_points : HashSet<CoordinatePoint> = HashSet::new();

        for check_point_x in self.top_left.get_x()..=self.bottom_right.get_x() {
            for check_point_y in self.top_left.get_y()..=self.bottom_right.get_y() {

                let check_point : CoordinatePoint = CoordinatePoint::new(check_point_x, check_point_y);
                let sum_distances : CoordinateUnit = self.full_mapping.get(&check_point).unwrap()
                    .iter().map(|(_, &distance)| distance).sum();
                if sum_distances < threshold { valid_points.insert(check_point); }

            }
        }
        
        return valid_points;
    }
}