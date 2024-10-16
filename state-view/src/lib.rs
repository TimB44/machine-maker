//! Visual context for state machines

use engine_room::StateMachine;

pub struct Viewer {
    //TODO: fix after refactor StateMachine to be an enum
    machine: Box<dyn StateMachine>,
}

enum EdgeParseError {}

enum Edge {}

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
