//! Visual context for state machines

mod point;

use engine_room::dfa::{Dfa, DfaBuilder};
use point::Point;
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct Viewer {
    #[ts(skip)]
    machine: BuildableMachine,
    states_names: Vec<String>,
    state_pos: Vec<Point>,
    edge_visuals: Vec<VisualEdgeType>,
}

enum EdgeParseError {}

enum Edge {}

#[derive(TS)]
#[ts(export)]
enum VisualEdgeType {
    Straight,
    Angle,
    Bezier {
        start_dx: f32,
        start_dy: f32,
        end_dx: f32,
        end_dy: f32,
    },
}

enum BuildableMachine {
    Dfa(BuildableDfa),
}

enum BuildableDfa {
    Built(Dfa),
    UnBuilt(DfaBuilder),
}
