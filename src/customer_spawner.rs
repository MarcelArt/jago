use std::ops::DerefMut;

use godot::{
    classes::{INode2D, Node2D, RandomNumberGenerator, ResourceLoader, Timer},
    prelude::*,
};

use crate::customer::Customer;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct CustomerSpawner {
    base: Base<Node2D>,
    timer: Option<Gd<Timer>>,
    customer_scene: Gd<PackedScene>,
    spawn_chance: f32,

    // Change or add your own properties here
    #[export]
    spawn_points: Array<Gd<Node2D>>,
}

#[godot_api]
impl INode2D for CustomerSpawner {
    fn init(base: Base<Node2D>) -> Self {
        let customer_scene = Some(
            ResourceLoader::singleton()
                .load("res://scenes/customer.tscn")
                .unwrap()
                .cast::<PackedScene>(),
        );

        Self {
            base,
            timer: None,
            spawn_points: Array::new(),
            customer_scene: customer_scene.unwrap(),
            spawn_chance: 0.3,
        }
    }

    fn ready(&mut self) {
        self.timer = Some(self.base().get_node_as("Timer"));

        let timer = self.timer.as_ref().unwrap();
        timer
            .signals()
            .timeout()
            .connect_other(&*self, Self::_on_timer_timeout);
    }

    fn process(&mut self, _delta: f64) {}
}

#[godot_api]
impl CustomerSpawner {
    fn spawn_customer(&mut self) {
        let mut rng = RandomNumberGenerator::new_gd();
        let spawn_rng = rng.randf();
        if spawn_rng > self.spawn_chance {
            return;
        }

        let mut customer = self.customer_scene
            .instantiate()
            .unwrap()
            .cast::<Customer>();

        let i = rng.randi_range(0, 1) as usize;
        let spawn_points = self.get_spawn_points();
        let spawn_point = spawn_points.get(i).unwrap();
        customer.bind_mut().set_walk_direction(if i == 1 { Vector2::RIGHT } else { Vector2::LEFT });

        customer.set_position(spawn_point.get_position());
        self.base_mut().add_child(Some(&customer));
    }

    #[func]
    fn _on_timer_timeout(&mut self) {
        self.spawn_customer();
    }

}
