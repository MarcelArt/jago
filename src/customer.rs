use godot::{classes::{Area2D, CharacterBody2D, ICharacterBody2D, VisibleOnScreenNotifier2D}, prelude::*};

use crate::utils::rng;

enum CustomerState {
    Walking,
    Queue,
    Leaving,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Customer {
    base: Base<CharacterBody2D>,
    speed_multiplier: f32,
    visibility_notifier: Option<Gd<VisibleOnScreenNotifier2D>>,
    customer_state: CustomerState,
    walk_direction: Vector2,

    // Change or add your own properties here
    #[export]
    walk_speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Customer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            walk_speed: 100.0,
            speed_multiplier: 100.0,
            visibility_notifier: None,
            customer_state: CustomerState::Walking,
            walk_direction: Vector2::LEFT,
        }
    }

    fn ready(&mut self) {
        godot_print!("register visibility notifier");
        self.visibility_notifier = Some(self.base().get_node_as("VisibleOnScreenNotifier2D"));
        let notifier = self.visibility_notifier.as_ref().unwrap();
        notifier
            .signals()
            .screen_exited()
            .connect_other(&*self, Self::_on_visibility_notifier_screen_exited);
        godot_print!("finished registering visibility notifier");
    }

    fn process(&mut self, _delta: f64) {
        match self.customer_state {
            CustomerState::Walking => self.walk(_delta),
            CustomerState::Queue => {} // Implement queue behavior here
            CustomerState::Leaving => self.walk(_delta),
        }
    }  
}

impl Customer {
    fn walk(&mut self, delta: f64) {
        let velocity = self.walk_direction * self.walk_speed * delta as f32 * self.speed_multiplier;

        let mut customer = self.base_mut();
        customer.set_velocity(velocity);
        customer.move_and_slide();
    }

    fn _on_visibility_notifier_screen_exited(&mut self) {
        if let CustomerState::Leaving = self.customer_state {
            self.base_mut().queue_free();
        }
        self.base_mut().queue_free();
    }

    pub fn set_walk_direction(&mut self, direction: Vector2) {
        self.walk_direction = direction;
    }

    pub fn decide_to_queue(&mut self, _area_2d: Gd<Area2D>) {
        godot_print!("Deciding to queue");
        let is_wanting_to_queue = rng::check_chance(30.0);
        if is_wanting_to_queue {
            self.customer_state = CustomerState::Queue;
            godot_print!("Decided to queue");
        } else {
            self.customer_state = CustomerState::Leaving;
            godot_print!("Decided to leave");
        }
    }
}