use crate::types::LayersFill;

impl LayersFill {
    pub fn new() -> LayersFill {
        LayersFill {
            values_preset: vec![vec![100], vec![200], vec![300, 320]],
            is_preset_ordered: true,
            values_divation: Some(10.0),
            values_smooth: None,
            values_offset: None,
        }
    }
}

impl LayersFill {
    pub fn set_values_preset(self: &mut Self, values: Vec<Vec<i32>>) -> Result<(), &'static str> {
        if values.len() < 1 { return Err("Vector must contain at least one element")}
        for value in &values {
            if value.len() > 2 { return Err("Every sub vector can contain only 1 or 2 elements")}
        }
        self.values_preset = values;
        Ok(())
    }

    pub fn values_preset(self: &Self) -> &Vec<Vec<i32>> {
        return &self.values_preset
    }

    pub fn set_is_preset_odreder(self: &mut Self, state: bool) {
        self.is_preset_ordered = state;
    }

    pub fn is_preset_ordered(self: &Self) -> bool {
        return self.is_preset_ordered
    }

    pub fn set_values_divation(self: &mut Self, divation: Option<f32>) -> Result<(), &'static str> {
        if divation.unwrap_or(1.0) <= 0.0 { return Err("divation must be positive") };
        self.values_divation = divation;
        Ok(())
    }

    pub fn values_divation(self: &Self) -> Option<f32> {
        return self.values_divation
    }

    pub fn set_values_smooth(self: &mut Self, smooth: Option<u32>) {
        self.values_smooth = smooth;
    }

    pub fn values_smooth(self: &Self) -> Option<u32> {
        return self.values_smooth
    }
    
    pub fn set_values_offset(self: &mut Self, offset: Option<u32>) {
        self.values_offset = offset;
    }

    pub fn values_offset(self: &Self) -> Option<u32> {
        return self.values_offset
    }
}
