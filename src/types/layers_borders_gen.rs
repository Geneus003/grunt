use crate::types::LayersBorder;

impl LayersBorder {
    pub fn new() -> LayersBorder {
        LayersBorder {
            border_deviation: 0.0,
            border_mod_func: None,
            border_type: String::from("random"),
            border_max_step: None,
            border_step_prob: Some(0.5),
            layers_same_deviation: false,
        }
    }
}

impl LayersBorder {
    pub fn set_border_deviation(self: &mut Self, border_deviation: f32) -> Result<(), &'static str> {
        if border_deviation < 0.0 {
            return Err("Border deviation can't be negative.")
        }
        self.border_deviation = border_deviation;
        Ok(())
    }

    pub fn border_deviation(self: &Self) -> f32 {
        self.border_deviation
    }

    pub fn set_border_mod_func(self: &mut Self, mod_func: Option<fn(usize, usize, usize, i32) -> i32>) {
        self.border_mod_func = mod_func;
    }

    pub fn border_mod_func(self: &Self) -> Option<fn(usize, usize, usize, i32) -> i32> {
        self.border_mod_func
    }

    pub fn set_border_type(self: &mut Self, border_type: String) -> Result<(), &'static str> {
        match border_type.as_str() {
            "random" => (),
            _ => return Err("border_type can be next: random"),
        }
        Ok(())
    }

    pub fn border_type(self: &Self) -> &String {
        &self.border_type
    }

    pub fn set_border_max_step(self: &mut Self, max_step: Option<i32>) {
        self.border_max_step = max_step
    }

    pub fn border_max_step(self: &Self) -> Option<i32> {
        self.border_max_step
    }

    pub fn set_border_step_prob(self: &mut Self, prob: Option<f32>) -> Result<(), &'static str> {
        if prob.is_none() {
            self.border_step_prob = prob
        } else {
            let prob = (prob.unwrap() * 1000.0).round() / 1000.0;
            if 0.0 < prob || prob > 1.0 {
                return Err("Probability must be between 0.0 and 1.0");
                
            }
            self.border_step_prob = Some(prob)
        }
        Ok(())
    }

    pub fn border_step_prob(self: &Self) -> Option<f32> {
        self.border_step_prob
    }

    pub fn set_layers_same_deviation(self: &mut Self, same_deviation: bool) {
        self.layers_same_deviation = same_deviation
    } 

    pub fn layers_same_deviation(self: &Self) -> bool {
        self.layers_same_deviation
    } 
}
