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

pub fn randf(from: f32, to: f32) -> f32 {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randf_range(from, to)
}

pub fn check_chance(percent: f32) -> bool {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randf() < (percent / 100.0)
}

pub fn randi(from: i32, to: i32) -> i32 {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randi_range(from, to)
}