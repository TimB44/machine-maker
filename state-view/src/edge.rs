pub trait LogicalEdge {
    fn to_string(&self) -> String;
    fn input_count(&self) -> usize;
    fn input_labels(&self) -> &'static [&'static str];
}

#[derive(Debug, Clone)]
pub enum EdgeType {
    SingleChar(String),
}

pub enum EdgeParseError {}

#[derive(Debug, Clone)]
pub enum VisualEdge {
    Straight,
    Angle,
    Bezier {
        start_dx: f32,
        start_dy: f32,
        end_dx: f32,
        end_dy: f32,
    },
}

const SINGLE_CHAR_LABELS: [&'static str; 1] = ["Character"];
impl LogicalEdge for EdgeType {
    fn to_string(&self) -> String {
        match self {
            EdgeType::SingleChar(s) => s.to_string(),
        }
    }

    fn input_count(&self) -> usize {
        match self {
            EdgeType::SingleChar(_) => 1,
        }
    }

    fn input_labels(&self) -> &'static [&'static str] {
        match self {
            EdgeType::SingleChar(_) => &SINGLE_CHAR_LABELS,
        }
    }
}
