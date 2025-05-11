use anyhow::Result;
use serde::Serialize;
use sgp4::{Constants, Elements, MinutesSinceEpoch};

#[derive(Serialize)]
pub struct Point {
    pub t: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn propagate(el: &Elements) -> Result<Vec<Point>> {
    let consts = Constants::from_elements(el)?;

    let mut pts = Vec::new();
    for m in (0..=90).step_by(5) {
        let state = consts.propagate(MinutesSinceEpoch(m as f64))?;
        let p = state.position;              // [f64; 3]
        pts.push(Point { t: m as i32, x: p[0], y: p[1], z: p[2] });
    }
    Ok(pts)
}
