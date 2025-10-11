use godot::{classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, VisibleOnScreenNotifier2D}, prelude::*};

use crate::utils::rng;

enum CustomerState {
    Walking,
    Waiting,
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
    animated_sprite: Option<Gd<AnimatedSprite2D>>,

    // Change or add your own properties here
    #[export]
    walk_speed: f32,
    #[export]
    desire: f32,
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
            animated_sprite: None,
            desire: 30.0,
        }
    }

    fn ready(&mut self) {
        self.visibility_notifier = Some(self.base().get_node_as("VisibleOnScreenNotifier2D"));
        let notifier = self.visibility_notifier.as_ref().unwrap();
        notifier
            .signals()
            .screen_exited()
            .connect_other(&*self, Self::_on_visibility_notifier_screen_exited);

        self.animated_sprite = Some(self.base().get_node_as("Sprite2D"));
        let animated_sprite = self.animated_sprite.as_mut().unwrap();
        animated_sprite.play();
    }

    fn process(&mut self, _delta: f64) {
        match self.customer_state {
            CustomerState::Walking => self.walk(_delta),
            CustomerState::Waiting => {} // Implement queue behavior here
            CustomerState::Leaving => self.walk(_delta),
        }
    }  
}

#[godot_api]
impl Customer {
    #[signal]
    pub fn on_make_order(customer: Gd<Customer>,amount: i32);

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
        if self.walk_direction == Vector2::RIGHT {
            let animated_sprite = self.animated_sprite.as_mut().unwrap();
            animated_sprite.set_flip_h(true);
        }
    }

    #[func]
    pub fn decide_to_queue(&mut self, _body: Gd<Node2D>) {
        let gd_self = self.to_gd();
        let body: Gd<Customer> = _body.cast();
        
        if body != gd_self {
            return;
        }

        let is_buying = rng::check_chance(self.desire);
        if is_buying {
            self.customer_state = CustomerState::Waiting;
            self.make_order();
        } else {
            self.customer_state = CustomerState::Leaving;
        }
    }

    pub fn make_order(&mut self) {
        let gd_self = self.to_gd();
        self.signals().on_make_order().emit(&gd_self, 1);
    }

    pub fn complete_order(&mut self, _is_bought: bool) {
        self.customer_state = CustomerState::Leaving;
    }
}