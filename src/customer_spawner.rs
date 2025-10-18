
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
    cart_area: Option<Gd<Area2D>>,
    game_manager: Option<Gd<SellingPhase>>,
    
    // Change or add your own properties here
    #[export]
    spawn_points: Array<Gd<Node2D>>,
    #[export]
    spawn_chance: f32,
    #[export]
    min_spawn_y: f32,
    #[export]
    max_spawn_y: f32,
    #[export]
    customer_scenes: Array<Gd<PackedScene>>,
}

#[godot_api]
impl INode2D for CustomerSpawner {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            timer: None,
            cart_area: None,
            game_manager: None,
            spawn_points: Array::new(),
            customer_scenes: Array::new(),
            spawn_chance: 30.0, // 30% chance to spawn each interval
            min_spawn_y: 92.0,
            max_spawn_y: 94.0,
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

        let game_manager = self.game_manager.as_ref().unwrap();
        game_manager.signals()
            .on_toggle_fast_forward()
            .connect_other(&*self, Self::update_timer_speed);
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

        let i = rng::randi(0, self.get_customer_scenes().len() as i32 - 1);
        let customer_scene = self.get_customer_scenes().get(i as usize).unwrap();

        // init customer
        let mut gd_customer = customer_scene
            .instantiate()
            .unwrap()
            .cast::<Customer>();
        
        // set spawn position
        let i = rng::coin_toss() as usize;
        let spawn_points = self.get_spawn_points();
        let spawn_point = spawn_points.get(i).unwrap();
        let y = rng::randf(self.min_spawn_y, self.max_spawn_y);
        let spawn_coord = Vector2::new(spawn_point.get_position().x, y);
        gd_customer.set_position(spawn_coord);
        
        // spawn customer
        self.base_mut().add_child(Some(&gd_customer));
        
        // set walk direction
        let mut customer = gd_customer.bind_mut();
        customer.set_walk_direction(if i == 1 { Vector2::RIGHT } else { Vector2::LEFT });

        // register cart area signal
        let cart_area = self.cart_area.as_ref().unwrap();
        cart_area
            .signals()
            .body_entered()
            .connect_other(&*customer, Customer::decide_to_queue);

        // register customer making order signal
        let game_manager = self.game_manager.as_ref().unwrap();
        customer
            .signals()
            .on_make_order()
            .connect_other(&*game_manager, SellingPhase::update_orders);

        game_manager.signals()
            .on_toggle_fast_forward()
            .connect_other(&*customer, Customer::on_toggle_fast_forward);
    }

    #[func]
    fn _on_timer_timeout(&mut self) {
        self.spawn_customer();
    }

    fn update_timer_speed(&mut self, ff_speed: f64) {
        let timer = self.timer.as_mut().unwrap();
        let time_sec = 1.0 / ff_speed;
        timer.set_wait_time(time_sec);
    }
}
