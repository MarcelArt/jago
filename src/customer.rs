use godot::{classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Sprite2D, VisibleOnScreenNotifier2D}, global::clampf, prelude::*};

use crate::{customer_variant::CustomerVariant, enums::customer_feedback::CustomerFeedback, singletons::game_data::GameDataSingleton, utils::rng};

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
    variant: Option<Gd<CustomerVariant>>,
    #[export]
    love_bubble: Option<Gd<Sprite2D>>,
    #[export]
    like_bubble: Option<Gd<Sprite2D>>,
    #[export]
    dislike_bubble: Option<Gd<Sprite2D>>,
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
            variant: None,
            love_bubble: None,
            like_bubble: None,
            dislike_bubble: None,
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

        let is_buying = self.should_buy();
        godot_print!("Customer deciding to queue: {}", is_buying);
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

    pub fn complete_order(&mut self, is_bought: bool) {
        self.customer_state = CustomerState::Leaving;

        if !is_bought {
            return;
        }

        let mut game_data = GameDataSingleton::get_instance();
        let feedback: CustomerFeedback;
        {
            let recipe = &game_data.bind().recipe;
            let coffee_pref = self.get_variant().unwrap().bind().get_coffee_pref();
            let milk_pref = self.get_variant().unwrap().bind().get_milk_pref();
            let sugar_pref = self.get_variant().unwrap().bind().get_sugar_pref();
    
            let diff_coffee = (recipe.coffee - coffee_pref).abs();
            let diff_milk = (recipe.milk - milk_pref).abs();
            let diff_sugar = (recipe.sugar - sugar_pref).abs();
    
            let score = 1.0 - ((diff_coffee/10.0 + diff_milk / 150.0 + diff_sugar / 15.0) / 3.0);
            let score = clampf(score as f64, 0.0, 1.0);
            
            if score > 0.85 {
                let mut love_bubble = self.get_love_bubble().unwrap();
                love_bubble.set_visible(true);
                feedback = CustomerFeedback::Love;
            } else if score > 0.5 {
                let mut like_bubble = self.get_like_bubble().unwrap();
                like_bubble.set_visible(true);
                feedback = CustomerFeedback::Like;
            } else {
                let mut dislike_bubble = self.get_dislike_bubble().unwrap();
                dislike_bubble.set_visible(true);
                feedback = CustomerFeedback::Dislike;
            }

            // TODO: comment when not needed
            godot_print!("===================Verdict?===================");
            godot_print!("Coffee/Pref: {}/{}", recipe.coffee, coffee_pref);
            godot_print!("Milk/Pref: {}/{}", recipe.milk, milk_pref);
            godot_print!("Sugar/Pref: {}/{}", recipe.sugar, sugar_pref);
            godot_print!("Score: {:#?}", score);
            godot_print!("Feedback: {:#?}", feedback);
            godot_print!("==============================================");
        }

        game_data.bind_mut().update_favorability(feedback);
    }

    fn should_buy(&mut self) -> bool {
        let game_data = GameDataSingleton::get_instance();
        
        let favorability_factor = game_data.bind().favorability;
        
        let price = game_data.bind().price;
        let price_factor = clampf(1.0 - ((price as f64 - 8.0) / 8.0), 0.2, 1.5) as f32;

        let base_chance = favorability_factor * price_factor;
        let roll = rng::randf(0.0, 1.0);

        // TODO: comment when not needed
        godot_print!("=================Should buy?==================");
        godot_print!("Favorability factor: {}", favorability_factor);
        godot_print!("Price factor ({}): {}", price, price_factor);
        godot_print!("Buy chance: {}", base_chance);
        godot_print!("RNG roll: {}", roll);
        godot_print!("==============================================");

        roll < base_chance
    }
}