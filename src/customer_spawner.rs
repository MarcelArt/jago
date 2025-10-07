
use godot::{
    classes::{Area2D, INode2D, Node2D, ResourceLoader, Timer},
    prelude::*,
};

use crate::{customer::Customer, get_node_by_abs_path, selling_phase::SellingPhase, utils::rng};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct CustomerSpawner {
    base: Base<Node2D>,
    timer: Option<Gd<Timer>>,
    customer_scene: Gd<PackedScene>,
    spawn_chance: f32,
    cart_area: Option<Gd<Area2D>>,
    game_manager: Option<Gd<SellingPhase>>,

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
            cart_area: None,
            game_manager: None,
            spawn_points: Array::new(),
            customer_scene: customer_scene.unwrap(),
            spawn_chance: 30.0, // 30% chance to spawn each interval
        }
    }

    fn ready(&mut self) {
        self.timer = Some(self.base().get_node_as("Timer"));
        self.cart_area = Some(self.base().get_parent().unwrap().get_node_as("Cart/Area2D"));
        self.game_manager = Some(get_node_by_abs_path!(self.base(), "SellingPhase"));

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
        // decide to spawn
        let is_spawning = rng::check_chance(self.spawn_chance);
        if !is_spawning {
            return;
        }

        // init customer
        let mut gd_customer = self.customer_scene
            .instantiate()
            .unwrap()
            .cast::<Customer>();
        
        // set spawn position
        let i = rng::coin_toss() as usize;
        let spawn_points = self.get_spawn_points();
        let spawn_point = spawn_points.get(i).unwrap();
        let y = rng::randf(200.0, 350.0);
        let spawn_coord = Vector2::new(spawn_point.get_position().x, y);
        gd_customer.set_position(spawn_coord);
        
        // spawn customer
        self.base_mut().add_child(Some(&gd_customer));
        
        // set walk direction
        let mut customer = gd_customer.bind_mut();
        customer.set_walk_direction(if i == 1 { Vector2::RIGHT } else { Vector2::LEFT });

        // register signal
        let cart_area = self.cart_area.as_ref().unwrap();
        cart_area
            .signals()
            .body_entered()
            .connect_other(&*customer, Customer::decide_to_queue);

        let game_manager = self.game_manager.as_ref().unwrap();
        customer
            .signals()
            .on_make_order()
            .connect_other(&*game_manager, SellingPhase::update_orders);
    }

    #[func]
    fn _on_timer_timeout(&mut self) {
        self.spawn_customer();
    }
}
