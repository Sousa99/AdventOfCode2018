use std::collections::HashMap;
use std::ops::{Add, Sub};
use core::cmp::Ordering;

// ======================================================== STRUCTS DEFINITIONS ========================================================

type CoordinateUnit = i64;
type CartID = usize;

#[derive(Clone, Copy)]
enum TrackType { StraightUpDown, StraightLeftRight, DiagonalUpDown, DiagonalDownUp, Intersection }
#[derive(Clone, Copy)]
enum CartDirection { Up, Down, Left, Right }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, Debug)]
pub struct Coordinate2D { x: CoordinateUnit, y: CoordinateUnit }
struct Track { track_type: TrackType }
struct Cart { id: CartID, position: Coordinate2D, cart_direction: CartDirection, current_turns: usize }
pub struct Simulator { iteration: usize, map: HashMap<Coordinate2D, Track>, carts: HashMap<Coordinate2D, Cart>, collisions: HashMap<Coordinate2D, Vec<usize>> }

// ======================================================== AUXILIARY FUNCTIONS== ======================================================

fn _convert_char_track_type(track_type_char: &char) -> Option<TrackType> {
    match track_type_char {
        '|'     => Some(TrackType::StraightUpDown),
        '-'     => Some(TrackType::StraightLeftRight),
        '\\'    => Some(TrackType::DiagonalUpDown),
        '/'     => Some(TrackType::DiagonalDownUp),
        '+'     => Some(TrackType::Intersection),
        _       => None
    }
}

fn _convert_char_cart_direction(cart_direction_char: &char) -> Option<CartDirection> {
    match cart_direction_char {
        '^'     => Some(CartDirection::Up),
        '>'     => Some(CartDirection::Right),
        '<'     => Some(CartDirection::Left),
        'v'     => Some(CartDirection::Down),
        _       => None
    }
}

fn _convert_track_type_char(track_type: &TrackType) -> char {
    match track_type {
        TrackType::StraightUpDown       => '|',
        TrackType::StraightLeftRight    => '-',
        TrackType::DiagonalUpDown       => '\\',
        TrackType::DiagonalDownUp       => '/',
        TrackType::Intersection         => '+',
    }
}

fn _convert_cart_direction_char(cart_direction: &CartDirection) -> char {
    match cart_direction {
        CartDirection::Up       => '^',
        CartDirection::Right    => '>',
        CartDirection::Left     => '<',
        CartDirection::Down     => 'v',
    }
}


// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Coordinate2D {
    fn new(position_x: CoordinateUnit, position_y: CoordinateUnit) -> Coordinate2D {
        Coordinate2D { x: position_x, y: position_y }
    }

    pub fn get_x(&self) -> CoordinateUnit { self.x }
    pub fn get_y(&self) -> CoordinateUnit { self.y }
}

impl Add for Coordinate2D {
    type Output = Coordinate2D;
    fn add(self, other: Coordinate2D) -> Coordinate2D { Coordinate2D { x: self.x + other.x, y: self.y + other.y } }
}

impl Sub for Coordinate2D {
    type Output = Coordinate2D;
    fn sub(self, other: Coordinate2D) -> Coordinate2D { Coordinate2D { x: self.x - other.x, y: self.y - other.y } }
}

impl PartialOrd for Coordinate2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_y() == other.get_y() {
            return Some(self.get_x().cmp(&other.get_x()));
        }
        Some(self.get_y().cmp(&other.get_y()))
    }
}

impl Track {
    fn new(track_type: TrackType) -> Track {
        Track { track_type: track_type }
    }

    fn connetable_any(&self) -> bool { matches!(self.track_type, TrackType::DiagonalDownUp) || matches!(self.track_type, TrackType::DiagonalUpDown) || matches!(self.track_type, TrackType::Intersection) }
    fn connectable_opt_under(&self) -> bool { self.connetable_any() || matches!(self.track_type, TrackType::StraightUpDown) }
    fn connectable_opt_above(&self) -> bool { self.connetable_any() || matches!(self.track_type, TrackType::StraightUpDown) }
    fn connectable_opt_left(&self) -> bool { self.connetable_any() || matches!(self.track_type, TrackType::StraightLeftRight) }
    fn connectable_opt_right(&self) -> bool { self.connetable_any() || matches!(self.track_type, TrackType::StraightLeftRight) }

    fn connectable_need_under(&self) -> bool { matches!(self.track_type, TrackType::StraightUpDown) || matches!(self.track_type, TrackType::Intersection) }
    fn connectable_need_above(&self) -> bool { matches!(self.track_type, TrackType::StraightUpDown) || matches!(self.track_type, TrackType::Intersection) }
    fn connectable_need_left(&self) -> bool { matches!(self.track_type, TrackType::StraightLeftRight) || matches!(self.track_type, TrackType::Intersection) }
    fn connectable_need_right(&self) -> bool { matches!(self.track_type, TrackType::StraightLeftRight) || matches!(self.track_type, TrackType::Intersection) }
}

impl Cart {
    fn new(id: CartID, position: Coordinate2D, cart_direction: CartDirection) -> Cart {
        Cart { id: id, position: position, cart_direction: cart_direction, current_turns: 0 }
    }

    fn get_next_position(&self) -> Coordinate2D {
        let position_variation : Coordinate2D = match self.cart_direction {
            CartDirection::Up       => Coordinate2D::new( 0, -1),
            CartDirection::Down     => Coordinate2D::new( 0,  1),
            CartDirection::Left     => Coordinate2D::new(-1,  0),
            CartDirection::Right    => Coordinate2D::new( 1,  0),
        };

        return self.position + position_variation;
    }

    fn move_cart(&mut self, position: Coordinate2D, position_track_type: TrackType) {

        self.position = position;
        self.cart_direction = match (self.cart_direction, position_track_type) {
            (CartDirection::Up, TrackType::StraightUpDown)                                      => CartDirection::Up,
            (CartDirection::Up, TrackType::StraightLeftRight)                                   => panic!("🚨 Cart movement is illegal at '({}, {})'!", position.get_x(), position.get_y()),
            (CartDirection::Up, TrackType::DiagonalUpDown)                                      => CartDirection::Left,
            (CartDirection::Up, TrackType::DiagonalDownUp)                                      => CartDirection::Right,
            (CartDirection::Up, TrackType::Intersection)        if self.current_turns % 3 == 0  => CartDirection::Left,
            (CartDirection::Up, TrackType::Intersection)        if self.current_turns % 3 == 1  => CartDirection::Up,
            (CartDirection::Up, TrackType::Intersection)        if self.current_turns % 3 == 2  => CartDirection::Right,
            
            (CartDirection::Down, TrackType::StraightUpDown)                                    => CartDirection::Down,
            (CartDirection::Down, TrackType::StraightLeftRight)                                 => panic!("🚨 Cart movement is illegal at '({}, {})'!", position.get_x(), position.get_y()),
            (CartDirection::Down, TrackType::DiagonalUpDown)                                    => CartDirection::Right,
            (CartDirection::Down, TrackType::DiagonalDownUp)                                    => CartDirection::Left,
            (CartDirection::Down, TrackType::Intersection)      if self.current_turns % 3 == 0  => CartDirection::Right,
            (CartDirection::Down, TrackType::Intersection)      if self.current_turns % 3 == 1  => CartDirection::Down,
            (CartDirection::Down, TrackType::Intersection)      if self.current_turns % 3 == 2  => CartDirection::Left,
            
            (CartDirection::Left, TrackType::StraightUpDown)                                    => panic!("🚨 Cart movement is illegal at '({}, {})'!", position.get_x(), position.get_y()),
            (CartDirection::Left, TrackType::StraightLeftRight)                                 => CartDirection::Left,
            (CartDirection::Left, TrackType::DiagonalUpDown)                                    => CartDirection::Up,
            (CartDirection::Left, TrackType::DiagonalDownUp)                                    => CartDirection::Down,
            (CartDirection::Left, TrackType::Intersection)      if self.current_turns % 3 == 0  => CartDirection::Down,
            (CartDirection::Left, TrackType::Intersection)      if self.current_turns % 3 == 1  => CartDirection::Left,
            (CartDirection::Left, TrackType::Intersection)      if self.current_turns % 3 == 2  => CartDirection::Up,
            
            (CartDirection::Right, TrackType::StraightUpDown)                                   => panic!("🚨 Cart movement is illegal at '({}, {})'!", position.get_x(), position.get_y()),
            (CartDirection::Right, TrackType::StraightLeftRight)                                => CartDirection::Right,
            (CartDirection::Right, TrackType::DiagonalUpDown)                                   => CartDirection::Down,
            (CartDirection::Right, TrackType::DiagonalDownUp)                                   => CartDirection::Up,
            (CartDirection::Right, TrackType::Intersection)     if self.current_turns % 3 == 0  => CartDirection::Up,
            (CartDirection::Right, TrackType::Intersection)     if self.current_turns % 3 == 1  => CartDirection::Right,
            (CartDirection::Right, TrackType::Intersection)     if self.current_turns % 3 == 2  => CartDirection::Down,
            
            (_, TrackType::Intersection)                                                        => panic!("🚨 Never will be called!"),
        };

        if matches!(position_track_type, TrackType::Intersection) {
            self.current_turns = self.current_turns + 1;
        }
    }
}

impl Simulator {
    pub fn  new(initial_map: Vec<Vec<char>>) -> Simulator {

        let mut map_track : HashMap<Coordinate2D, Track> = HashMap::new();
        let mut carts : HashMap<Coordinate2D, Cart> = HashMap::new();

        for (index_line, line) in initial_map.into_iter().enumerate() {
            for (index_char, char) in line.into_iter().enumerate() {

                let position : Coordinate2D = Coordinate2D::new(index_char as CoordinateUnit, index_line as CoordinateUnit);
                let track_type_option : Option<TrackType> = _convert_char_track_type(&char);
                let cart_direction_option : Option<CartDirection> = _convert_char_cart_direction(&char);

                match (track_type_option, cart_direction_option) {
                    (Some(track_type),  _                   ) => { map_track.insert(position, Track::new(track_type)); },
                    (_,                 Some(cart_direction)) => { carts.insert(position, Cart::new(carts.len(), position, cart_direction)); },
                    (_,                 _                   ) => (),
                };
            }
        }

        for (&position, _) in carts.iter() {
            let above_track_option  = map_track.get(&(position - Coordinate2D::new( 0, -1)));
            let under_track_option  = map_track.get(&(position - Coordinate2D::new( 0,  1)));
            let left_track_option   = map_track.get(&(position - Coordinate2D::new(-1,  0)));
            let right_track_option  = map_track.get(&(position - Coordinate2D::new( 1,  0)));

            let opt_above = above_track_option.is_some() && above_track_option.unwrap().connectable_opt_under();
            let opt_under = under_track_option.is_some() && under_track_option.unwrap().connectable_opt_above();
            let opt_left = left_track_option.is_some() && left_track_option.unwrap().connectable_opt_right();
            let opt_right = right_track_option.is_some() && right_track_option.unwrap().connectable_opt_left();

            let need_above = above_track_option.is_some() && above_track_option.unwrap().connectable_need_under();
            let need_under = under_track_option.is_some() && under_track_option.unwrap().connectable_need_above();
            let need_left = left_track_option.is_some() && left_track_option.unwrap().connectable_need_right();
            let need_right = right_track_option.is_some() && right_track_option.unwrap().connectable_need_left();

            let current_position_track_type : TrackType = match (need_above, need_under, need_left, need_right, opt_above, opt_under, opt_left, opt_right) {
                (true ,  true,  true,  true,     _,     _,     _,     _) => TrackType::Intersection,
                (true ,  true, false, false,     _,     _,     _,     _) => TrackType::StraightUpDown,
                (false, false,  true,  true,     _,     _,     _,     _) => TrackType::StraightLeftRight,
                (true , false,  true, false,     _,     _,     _,     _) => TrackType::DiagonalDownUp,
                (true , false, false,  true,     _,     _,     _,     _) => TrackType::DiagonalUpDown,
                (false,  true,  true, false,     _,     _,     _,     _) => TrackType::DiagonalUpDown,
                (false,  true, false,  true,     _,     _,     _,     _) => TrackType::DiagonalDownUp,

                (    _,     _,     _,     _,  true,  true, false, false) => TrackType::StraightUpDown,
                (    _,     _,     _,     _,  false, false, true,  true) => TrackType::StraightLeftRight,

                _ => panic!("🚨 Not recognized as a valid surrounding for track in '({}, {})'", position.get_x(), position.get_y()),
            };

            map_track.insert(position, Track::new(current_position_track_type));
        }

        return Simulator { iteration: 0, map: map_track, carts: carts, collisions: HashMap::new() }
    }

    pub fn get_iteration(&self) -> usize { self.iteration }
    pub fn get_crashes(&self) -> Vec<(&Coordinate2D, &Vec<usize>)> { self.collisions.iter().collect() }
    pub fn get_carts_positions(&self) -> Vec<&Coordinate2D> { self.carts.iter().map(|(position, _)| position).collect() }

    pub fn run_iteration(&mut self) {

        let mut new_carts : HashMap<Coordinate2D, Cart> = HashMap::new();
        let mut carts_pos_to_iter : Vec<Coordinate2D> = self.carts.iter()
            .map(|(position, _)| position.clone())
            .collect();
        carts_pos_to_iter.sort();

        for cart_pos_to_iter in carts_pos_to_iter.iter() {
            let cart_option : Option<Cart> = self.carts.remove(cart_pos_to_iter);
            if cart_option.is_none() { continue; }

            let mut cart : Cart = cart_option.unwrap();
            let next_position_cart : Coordinate2D = cart.get_next_position();
            let track_in_next_position_cart : &Track = self.map.get(&next_position_cart).unwrap();

            cart.move_cart(next_position_cart, track_in_next_position_cart.track_type);

            // Check for collisions and deal with them correctly
            if false && self.collisions.contains_key(&next_position_cart) {
                // Not suposed to work like this
                let current_collisions = self.collisions.get_mut(&next_position_cart).unwrap();
                current_collisions.push(cart.id);
            } else if new_carts.contains_key(&next_position_cart) {
                let other_car = new_carts.remove(&next_position_cart).unwrap();
                if self.collisions.contains_key(&next_position_cart) {
                    let current_collisions = self.collisions.get_mut(&next_position_cart).unwrap();
                    current_collisions.push(cart.id);
                    current_collisions.push(other_car.id);
                } else { self.collisions.insert(next_position_cart, vec![cart.id, other_car.id]); }
            } else if self.carts.contains_key(&next_position_cart) {
                let other_car = self.carts.remove(&next_position_cart).unwrap();
                if self.collisions.contains_key(&next_position_cart) {
                    let current_collisions = self.collisions.get_mut(&next_position_cart).unwrap();
                    current_collisions.push(cart.id);
                    current_collisions.push(other_car.id);
                } else { self.collisions.insert(next_position_cart, vec![cart.id, other_car.id]); }
            } else { new_carts.insert(next_position_cart, cart); }
        }

        self.iteration = self.iteration + 1;
        self.carts = new_carts;
    }

    fn _get_limits_map(&self) -> (Coordinate2D, Coordinate2D) {
        let positions_map = self.map.iter().map(|(position, _)| position).collect::<Vec<&Coordinate2D>>();
        let positions_cart = self.carts.iter().map(|(position, _)| position).collect::<Vec<&Coordinate2D>>();
        let positions_crashes = self.collisions.iter().map(|(position, _)| position).collect::<Vec<&Coordinate2D>>();

        let final_iterator = positions_map.iter().chain(positions_cart.iter()).chain(positions_crashes.iter());
        let min_position_x = final_iterator.clone().map(|position| position.get_x()).min().unwrap();
        let min_position_y = final_iterator.clone().map(|position| position.get_y()).min().unwrap();
        let max_position_x = final_iterator.clone().map(|position| position.get_x()).max().unwrap();
        let max_position_y = final_iterator.clone().map(|position| position.get_y()).max().unwrap();

        return (Coordinate2D::new(min_position_x, min_position_y), Coordinate2D::new(max_position_x, max_position_y));
    }

    pub fn _print_map(&self, print_collisions: bool) -> String {

        let mut final_string : String = format!("🛒 Map on iteration '{}'\n", self.iteration);
        let limits = self._get_limits_map();
        for position_y in limits.0.get_y()..=limits.1.get_y() {
            for position_x in limits.0.get_x()..=limits.1.get_x() {
                let check_position : Coordinate2D = Coordinate2D::new(position_x, position_y);
                let track_option : Option<&Track> = self.map.get(&check_position);
                let cart_option : Option<&Cart> = self.carts.get(&check_position);
                let collision_place : bool = self.collisions.contains_key(&check_position);

                match (track_option, cart_option, collision_place) {
                    (_,             _,          true) if print_collisions   => final_string.push('X'),
                    (_,             Some(cart), _   )                       => final_string.push(_convert_cart_direction_char(&cart.cart_direction)),
                    (Some(track),   None,       _   )                       => final_string.push(_convert_track_type_char(&track.track_type)),
                    (None,          None,       _   )                       => final_string.push(' '),
                }
            }

            final_string.push('\n');
        }

        return final_string;
    }
}