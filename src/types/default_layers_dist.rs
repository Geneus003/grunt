use rand::Rng;

use crate::types::DefaultLayersDist;

impl DefaultLayersDist {
    pub fn new() -> DefaultLayersDist {
        DefaultLayersDist {
            layers_num: 3,
            max_layer_size: 100,
            min_layer_size:70,
            layers_sum: 230,
            layers_dist: vec![70, 90, 80],
        }
    }

    pub fn create_from_vec(layers_dist: Vec<u32>) -> Result<DefaultLayersDist, &'static str> {
        if layers_dist.is_empty() {
            return Err("Distibution of layers vec must contain at least one value");
        }

        let (mut min_layer_size, mut max_layer_size, mut layers_sum) = (u32::MAX, 0u32, 0u32);

        for el in &layers_dist {
            if *el < min_layer_size { min_layer_size = *el }
            if *el > max_layer_size { max_layer_size = *el }

            layers_sum = layers_sum.checked_add(*el).ok_or("Problem with calculating sum of layers: u32 overflow")?;
        }

        Ok(DefaultLayersDist {
            layers_num: layers_dist.len() as u32,
            max_layer_size,
            min_layer_size,
            layers_sum,
            layers_dist
        })
    }

    pub fn generate_from_params(
        layers_num: u32,
        min_layer_size: u32,
        max_layer_size: u32,
        layers_sum: Option<u32>
    ) -> Result<DefaultLayersDist, &'static str> {
        let layers = DefaultLayersDist::generate_layers_dist(layers_num, min_layer_size, max_layer_size, layers_sum);
        match layers {
            Err(err) => return Err(err),
            Ok(_) => Ok(DefaultLayersDist {
                layers_num,
                min_layer_size,
                max_layer_size,
                layers_sum: layers_sum.unwrap_or(layers.clone().unwrap().iter().sum()),
                layers_dist: layers.unwrap(),
            })
        }
    }

    // function params are named by DefaultLayersDist's first letters, e.g. ln - (l)ayers_(n)um
    pub fn generate_layers_dist(
        layers_num: u32,
        min_layer_size: u32,
        max_layer_size: u32,
        layers_sum: Option<u32>
    ) -> Result<Vec<u32>, &'static str> {
        match DefaultLayersDist::validate_params(layers_num, min_layer_size, max_layer_size, layers_sum) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let mut rng = rand::thread_rng();
        let mut layers: Vec<u32> = (0..layers_num).map(|_| rng.gen_range(min_layer_size..max_layer_size)).collect();

        if layers_sum.is_none() { return Ok(layers); }
        let layers_sum = layers_sum.unwrap();

        let mut points;
        let mut znak = true;
        if layers_sum > layers.iter().sum() {
            points = layers_sum - layers.iter().sum::<u32>();
        }
        else {
            znak = false;
            points = layers.iter().sum::<u32>() - layers_sum;
        }
        let mut tries: u32 = 0;

        while points != 0 && tries <= 2000 {
            let avg_layer_mod = points as u32 / layers_num;

            for i in 0..layers_num as usize {
                if znak {
                    if points + layers[i] <= max_layer_size {
                        layers[i] += points;
                        points = 0;
                        break;
                    }

                    let now_mod = if avg_layer_mod == 0 {
                        rng.gen_range(0..layers_num)
                    } else {
                        rng.gen_range(0..avg_layer_mod*2)
                    };

                    if layers[i] + now_mod <= max_layer_size {
                        layers[i] += now_mod;
                        points -= now_mod;
                    }
                } else {
                    if layers[i].checked_sub(points).unwrap_or(0) >= min_layer_size {
                        layers[i] -= points;
                        points = 0;
                        break;
                    }

                    let now_mod = if avg_layer_mod == 0 {
                        rng.gen_range(0..layers_num)
                    } else {
                        rng.gen_range(0..avg_layer_mod*2)
                    };

                    if layers[i].checked_sub(now_mod).unwrap_or(0) >= min_layer_size {
                        layers[i] -= now_mod;
                        points = points.checked_sub(now_mod).unwrap_or(0);
                    }
                }
            }

            tries += 1;
        }

        if points != 0 { 
            return Err("Something really gone wrong, this is program's fault, just try another arguments") 
        }
        Ok(layers)
    }

    fn validate_params(ln: u32, min_ls: u32, max_ls: u32, ls: Option<u32>) -> Result<(), &'static str> {
        if min_ls > max_ls {
            return Err("Max layer's size must be bigger than min layer's size")
        }

        if max_ls == 0 || ln == 0 || min_ls == 0 {
            return Err("Any argument must be bigger than zero")
        }

        if ls.is_some() {
            let ls = ls.unwrap();

            if ls == 0 {
                return Err("Any argument must be bigger than zero")
            }
            if max_ls * ln < ls {
                return Err("Impossible: (max layer's size) * (number of layers) must be bigger than (layers sum)");
            }

            if min_ls * ln > ls {
                return Err("Impossible: (min layer's size) * (number of layers) must be smaller than (layers sum)");
            }
        }

        Ok(())
    }
}

impl DefaultLayersDist {
    pub fn get_full_data(self: Self) -> (u32, u32, u32, u32, Vec<u32>) {
        (self.layers_num, self.max_layer_size, self.min_layer_size, self.layers_sum, self.layers_dist)
    }

    pub fn get_len(self: &Self) -> usize {
        self.layers_dist.len()
    }
}
