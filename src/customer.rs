use godot::{classes::{CharacterBody2D, ICharacterBody2D, VisibleOnScreenNotifier2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Customer {
    base: Base<CharacterBody2D>,
    speed_multiplier: f32,
    visibility_notifier: Option<Gd<VisibleOnScreenNotifier2D>>,

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
        }
    }

    fn ready(&mut self) {
        self.visibility_notifier = Some(self.base().get_node_as("VisibleOnScreenNotifier2D"));
        let notifier = self.visibility_notifier.as_ref().unwrap();
        notifier
            .signals()
            .screen_exited()
            .connect_other(&*self, Self::_on_visibility_notifier_screen_exited);
    }

    fn process(&mut self, _delta: f64) {
        self.move_toward_cart(_delta);
    }  
}     

impl Customer {
    fn move_toward_cart(&mut self, delta: f64) {
        let velocity = Vector2::LEFT * self.walk_speed * delta as f32 * self.speed_multiplier;

        let mut customer = self.base_mut();
        customer.set_velocity(velocity);
        customer.move_and_slide();
    }

    fn _on_visibility_notifier_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}