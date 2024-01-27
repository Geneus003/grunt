use crate::types::LayersFill;

impl LayersFill {
    pub fn new() -> LayersFill {
        LayersFill {
            values_preset: vec![vec![100, 100], vec![200, 200], vec![300, 300]],
            is_preset_ordered: true,
            values_divation: Some(10.0),
            values_smooth: None,
            values_offset: None,
        }
    }
}
