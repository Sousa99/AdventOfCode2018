use std::collections::HashMap;

// ======================================================== STRUCTS DEFINITIONS ========================================================

struct AccessibleVector<T> {
    _first_value: T,
    current_value: T,
    next_mappings: HashMap<T, T>,
    previous_mappings: HashMap<T, T>,
}

pub struct MarbleGame {
    number_players: usize,
    max_marble: usize,
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl<T : std::fmt::Display + Copy + PartialEq + Eq + std::hash::Hash> AccessibleVector<T> {
    fn new(first_value: T) -> AccessibleVector<T> {
        AccessibleVector {
            _first_value: first_value,
            current_value: first_value,
            next_mappings: vec![(first_value, first_value)].into_iter().collect(),
            previous_mappings: vec![(first_value, first_value)].into_iter().collect()
        }
    }

    fn transverse_forward_n(&mut self, number_jumps: usize) {
        for _ in 0..number_jumps {
            self.current_value = *self.next_mappings.get(&self.current_value).unwrap();
        }
    }
    fn transverse_backward_n(&mut self, number_jumps: usize) {
        for _ in 0..number_jumps {
            self.current_value = *self.previous_mappings.get(&self.current_value).unwrap();
        }
    }

    fn insert_after(&mut self, value: T) {
        let node_to_be_previous = self.current_value.clone();
        let node_to_be_next = *self.next_mappings.get(&self.current_value).unwrap();

        self.next_mappings.insert(node_to_be_previous, value);
        self.previous_mappings.insert(node_to_be_next, value);

        self.next_mappings.insert(value, node_to_be_next);
        self.previous_mappings.insert(value, node_to_be_previous);

        self.current_value = value;
    }

    fn remove_current(&mut self) -> T {
        let previous_current_value = self.current_value;

        let node_next = *self.next_mappings.get(&previous_current_value).unwrap();
        let node_previous = *self.previous_mappings.get(&previous_current_value).unwrap();

        self.next_mappings.remove(&previous_current_value);
        self.previous_mappings.remove(&previous_current_value);
        self.next_mappings.insert(node_previous, node_next);
        self.previous_mappings.insert(node_next, node_previous);

        self.current_value = node_next;
        return previous_current_value;
    }

    fn _print_formatted(&self) -> String {
        let mut print_array : Vec<String> = Vec::new();

        let mut iterator_value = self._first_value;
        let mut finished : bool = false;
        while !finished {
            
            if iterator_value != self.current_value { print_array.push(iterator_value.to_string()); }
            else { print_array.push(format!("({})", iterator_value.to_string())); };
            iterator_value = *self.next_mappings.get(&iterator_value).unwrap();
            if iterator_value == self._first_value { finished = true; }
        }
        

        return format!("[{}]", print_array.join(" "));
    }
}

impl MarbleGame {
    pub fn new(number_players: usize, max_marble: usize) -> MarbleGame {
        MarbleGame { number_players: number_players, max_marble: max_marble }
    }

    pub fn play_game(&self) -> HashMap<usize, Vec<i64>> {

        // Return values
        let mut player_scores : HashMap<usize, Vec<i64>> = (1..=self.number_players).map(|player| (player, Vec::new())).collect();
        
        let mut board_game : AccessibleVector<i64> = AccessibleVector::new(0);
        // Iterate Marbles and Players
        let mut current_player : usize = 1;
        for marble_to_place in 1..=self.max_marble {

            // Restore player index
            if current_player > self.number_players { current_player = 1 }

            // Effectively deal with plays
            if marble_to_place % 23 != 0 {
                for _ in 0..1 { board_game.transverse_forward_n(1); }
                board_game.insert_after(marble_to_place as i64);
            } else {
                for _ in 0..7 { board_game.transverse_backward_n(1); }
                let removed_marble : i64 = board_game.remove_current();
                let current_player_score = player_scores.get_mut(&current_player).unwrap();
                current_player_score.push(marble_to_place as i64);
                current_player_score.push(removed_marble);
            }

            //println!("[{}]: {}", current_player, board_game._print_formatted(current_index));
            // Update current player
            current_player = current_player + 1;
        }

        return player_scores;
    }
}