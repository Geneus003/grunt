use pyo3::prelude::*;

use crate::types::Axis;

#[pymethods]
impl Axis {
    pub fn new() -> Axis {
        Axis {
            start: 0.0,
            end: 100.0,
            step: Some(1.0),
            axis: Axis::calculate_axis(0.0, 100.0, 1.0),
        }
    }
}
