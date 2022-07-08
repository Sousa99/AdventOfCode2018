
// ======================================================== STRUCTS DEFINITIONS ========================================================

use std::collections::HashMap;

type Code = i64;
type ID = Code;

#[derive(Debug)]
struct Node {
    children: Vec<ID>,
    metadatas: Vec<Code>
}

pub struct SystemDecoder {
    codes: Vec<Code>,
    nodes: HashMap<ID, Node>,
}

// ====================================================== STRUCTS IMPLEMENTATIONS ======================================================

impl Node {
    fn get_node_value(&self, nodes: &HashMap<ID, Node>) -> i64 {
        if self.children.len() == 0 { self.metadatas.iter().sum() }
        else { self.metadatas.iter()
            .filter(|&meta_index| *meta_index != 0 && *meta_index <= self.children.len() as ID)
            .map(|&meta_index| self.children.get(meta_index as usize - 1).unwrap())
            .map(|children_id| nodes.get(children_id).unwrap())
            .map(|children| children.get_node_value(nodes))
            .sum()
        }
    }
}

impl SystemDecoder {
    pub fn new(codes: Vec<Code>) -> SystemDecoder {
        SystemDecoder { codes: codes, nodes: HashMap::new() }
    }

    pub fn decode_codes(&mut self) {

        let mut current_code_index : usize = 0;
        let mut current_node_id : ID = 0;
        let mut nodes : HashMap<ID, Node> = HashMap::new();
        while current_code_index < self.codes.len() {
            (current_code_index, current_node_id, nodes) = self.create_node(current_code_index, current_node_id, nodes);
        }

        self.nodes = nodes;
    }

    fn create_node(&self, mut current_codes_index: usize, mut current_node_id: ID, mut nodes: HashMap<ID, Node>) -> (usize, ID, HashMap<ID, Node>) {

        // Get mandatory information
        let node_id = current_node_id;
        let number_of_children = *self.codes.get(current_codes_index).unwrap() as usize;
        let number_of_metadatas = *self.codes.get(current_codes_index + 1).unwrap() as usize;
        current_codes_index = current_codes_index + 2;

        // Create children nodes
        let mut children_nodes : Vec<ID> = Vec::new();
        for _ in 0..number_of_children {
            current_node_id = current_node_id + 1;
            children_nodes.push(current_node_id);
            (current_codes_index, current_node_id, nodes) = self.create_node(current_codes_index, current_node_id, nodes);
        }
        
        // Get node metadata
        let mut metadatas : Vec<Code> = Vec::new();
        for _ in 0..number_of_metadatas {
            let current_metadata = *self.codes.get(current_codes_index).unwrap();
            metadatas.push(current_metadata);
            current_codes_index = current_codes_index + 1;
        }
        
        // Add newly created node
        let new_node : Node = Node { children: children_nodes, metadatas: metadatas };
        nodes.insert(node_id, new_node);
        return (current_codes_index, current_node_id, nodes);
    }

    pub fn sum_metadata(&self) -> Code {
        return self.nodes.iter()
            .map(|(_, node)| &node.metadatas)
            .flatten()
            .sum();
    }

    pub fn get_root_value(&self) -> i64 {
        let root_node = self.nodes.get(&0).unwrap();
        return root_node.get_node_value(&self.nodes);
    }
}