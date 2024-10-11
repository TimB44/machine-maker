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
}

pub trait Visual: StateMachine {
    fn parse_edge(&self, itmes: Vec<String>) -> Result<EdgeType, EdgeParseError>;
}
