use std::ops::Add;
use std::collections::HashMap;

use queue::Queue;

// ======================================================== STRUCTS DEFINITIONS ========================================================

type CoordinateUnit = i64;

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinate2D {
    pub x: CoordinateUnit,
    pub y: CoordinateUnit,
}

#[derive(Clone, Copy)]
pub struct PointDefinition {
    pub position: Coordinate2D,
    pub velocity: Coordinate2D,
}

pub struct Sky {
    iteration: usize,
    point_definitions: HashMap<usize, PointDefinition>,
    current_sky: HashMap<usize, Coordinate2D>,
    densities: Queue<f64>,
    print_size_factor: f64,
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Add for Coordinate2D {
    type Output = Coordinate2D;

    fn add(self, other: Coordinate2D) -> Coordinate2D {
        Coordinate2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sky {
    pub fn new(point_definitions: &Vec<PointDefinition>, queue_capacity: usize, print_size_factor: f64) -> Sky {
        Sky {
            iteration: 0,
            point_definitions: point_definitions.iter().enumerate()
                .map(|(index, point_definition)| (index, point_definition.clone()))    
                .collect(),
            current_sky: point_definitions.iter().enumerate()
                .map(|(index, point_definition)| (index, point_definition.position))
                .collect(),
            densities: Queue::with_capacity(queue_capacity),
            print_size_factor: print_size_factor,
        }
    }

    pub fn get_current_iteration(&self) -> usize { self.iteration }

    fn compute_sky_limits(&self) -> Option<(Coordinate2D, Coordinate2D)> {
        let mut limits : Option<(Coordinate2D, Coordinate2D)> = None;
        for (_, point) in self.current_sky.iter() {
            if limits.is_none() { limits = Some((point.clone(), point.clone())) }
            else {
                let limits_ref = limits.as_mut().unwrap();
                if point.x < limits_ref.0.x + 1 { limits_ref.0.x = point.x - 1; }
                if point.y < limits_ref.0.y + 1 { limits_ref.0.y = point.y - 1; }
                if point.x > limits_ref.1.x - 1 { limits_ref.1.x = point.x + 1; }
                if point.y > limits_ref.1.y - 1 { limits_ref.1.y = point.y + 1; }
            }
        }

        return limits;
    }

    fn get_density(&self) -> f64 {
        let sky_limits = self.compute_sky_limits().unwrap();
        let sky_width = (sky_limits.1.x - sky_limits.0.x) as f64;
        let sky_height = (sky_limits.1.y - sky_limits.0.y) as f64;

        let number_points = self.point_definitions.len() as f64;
        
        return number_points / (sky_width * sky_height);
    }

    pub fn worth_running(&self) -> bool {

        if self.densities.len() != self.densities.capacity().unwrap() { return true; }

        let densities_vec = self.densities.vec();
        return densities_vec.iter().zip(densities_vec.iter().skip(1))
            .any(|(before_density, after_density)| before_density < after_density);
    }

    pub fn run_iteration(&mut self) {

        self.iteration = self.iteration + 1;
        for (point_index, sky_point) in self.current_sky.iter_mut() {
            let point_definition = self.point_definitions.get(point_index).unwrap();
            *sky_point = *sky_point + point_definition.velocity;
        }

        self.densities.force_queue(self.get_density());
    }

    pub fn worth_printing(&self) -> bool {
        let sky_limits = self.compute_sky_limits().unwrap();
        let sky_width = (sky_limits.1.x - sky_limits.0.x) as f64;
        let sky_height = (sky_limits.1.y - sky_limits.0.y) as f64;

        let number_points = self.point_definitions.len() as f64;

        return sky_width + sky_height <= number_points * self.print_size_factor;
    }

    pub fn print_current_sky(&self) -> String {

        let sky_limits = self.compute_sky_limits().unwrap();
        let checked_points : Vec<&Coordinate2D> = self.current_sky.iter()
            .map(|(_, sky_point)| sky_point)
            .collect();

        let mut final_string : String = String::new();
        for check_point_y in sky_limits.0.y..=sky_limits.1.y {
            for check_point_x in sky_limits.0.x..=sky_limits.1.x {

                let check_point = Coordinate2D { x: check_point_x, y: check_point_y };
                if checked_points.contains(&&check_point) { final_string.push('x'); }
                else { final_string.push('.'); }
            }

            final_string.push('\n')
        }

        return final_string;
    }
}