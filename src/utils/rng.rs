use godot::{classes::RandomNumberGenerator, obj::NewGd};

pub fn coin_toss() -> i32 {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randi_range(0, 1)
}

#[allow(dead_code)]
pub fn coin_toss_bool() -> bool {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randi_range(0, 1) == 1
}

#[allow(dead_code)]
pub fn randf() -> f32 {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randf()
}

pub fn check_chance(percent: f32) -> bool {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randf() < (percent / 100.0)
}