//! Simple 2d Point

use ts_rs::TS;

/// Simple 2d Point
#[derive(Debug, Clone, TS)]
pub struct Point {
    x: f32,
    y: f32,
}
