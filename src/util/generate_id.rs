use rand::prelude::*;

pub fn generate_id() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
}

