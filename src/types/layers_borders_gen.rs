use crate::types::LayersBorder;

impl LayersBorder {
    pub fn new() -> LayersBorder {
        LayersBorder {
            border_divation: 0.0,
            border_mod_func: None,
            border_type: String::from("random"),
            border_max_step: None,
            layers_same_divation: false,
        }
    }

    pub fn set_border_divation(self: &mut Self, border_divation: f32) -> Result<(), &'static str> {
        if border_divation < 0.0 {
            return Err("Border divation can't be negative.")
        }
        self.border_divation = border_divation;
        return Ok(())
    }

    pub fn border_divation(self: &Self) -> f32 {
        return self.border_divation
    }

    pub fn set_border_mod_func(self: &mut Self, mod_func: Option<fn()>) {
        self.border_mod_func = mod_func;
    }

    pub fn set_border_type(self: &mut Self, border_type: String) -> Result<(), &'static str> {
        match border_type.as_str() {
            "random" => (),
            _ => return Err("border_type can be next: random"),
        }
        return Ok(())
    }

    pub fn border_type(self: &Self) -> &String {
        &self.border_type
    }

    pub fn set_border_max_step(self: &mut Self, max_step: Option<i32>) {
        self.border_max_step = max_step
    }

    pub fn border_max_step(self: &Self) -> Option<i32> {
        return self.border_max_step
    }

    pub fn set_layers_same_divation(self: &mut Self, same_divation: bool) {
        self.layers_same_divation = same_divation
    } 

    pub fn layers_same_divation(self: &Self) -> bool {
        return self.layers_same_divation
    } 
}
