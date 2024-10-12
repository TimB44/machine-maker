//! Visual context for state machines

pub mod edge;
pub mod point;

use edge::EdgeParseError;
use edge::EdgeType;
use edge::VisualEdge;
use engine_room::Machine;
use engine_room::StateMachine;
use point::Point;

#[derive(Debug, Clone)]
pub struct Viewer {
    machine: Machine,
    edges: Vec<(EdgeType, VisualEdge)>,
    state_names: Vec<String>,
    state_pos: Vec<Point>,
}

impl Viewer {
    pub fn new(
        machine: Machine,
        edges: Vec<(EdgeType, VisualEdge)>,
        state_names: Vec<String>,
        state_pos: Vec<Point>,
    ) -> Self {
        Self {
            machine,
            edges,
            state_names,
            state_pos,
        }
    }

    pub fn move_state(&mut self, state: u16, new_pos: Point) {
        self.state_pos[state as usize] = new_pos;
    }

    pub fn modify_edge_visuals(&mut self, edge_num: u16, new_visuals: VisualEdge) {
        self.edges[edge_num as usize].1 = new_visuals
    }

    pub fn add_state(self) -> Self {
        todo!()
    }
}

pub trait Visual: StateMachine {
    fn parse_edge(&self, itmes: Vec<String>) -> Result<EdgeType, EdgeParseError>;
}
