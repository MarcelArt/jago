use godot::{classes::{INode2D, Label, Node2D}, prelude::*};

use crate::customer::Customer;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SellingPhase {
    base: Base<Node2D>,
    clock_label: Option<Gd<Label>>,
    current_time: f64,
    end_time: f64,  // End time in minutes (e.g., 20:00 = 1200 minutes)
    is_day_over: bool,
    time_speed: f64, // Time speed multiplier
    
    // Change or add your own properties here
    #[export]
    time_multiplier: f64,
}

#[godot_api]
impl INode2D for SellingPhase {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            clock_label: None,
            current_time: (8 * 60) as f64, // Start at 8:00 AM
            time_speed: 1 as f64, // Normal speed
            end_time: (17 * 60) as f64, // End at 8:00 PM
            is_day_over: false,
            time_multiplier: 5 as f64, // Default time multiplier
        }
    }

    fn ready(&mut self) {
        self.clock_label = Some(self.base().get_node_as("UI/ClockLabel"));
    }

    fn process(&mut self, _delta: f64) {
        self.progress_time(_delta);
    }  
}     

impl SellingPhase {
    fn progress_time(&mut self, delta: f64) {
        if self.is_day_over {
            return;
        }
        let time_speed = self.time_speed;
        self.current_time += delta * time_speed * self.time_multiplier;
        if self.current_time >= self.end_time {
            self.current_time = self.end_time;
            self.is_day_over = true;
        }
        self.set_clock_label();
    }

    fn set_clock_label(&mut self) {
        let hours = (self.current_time / 60.0).floor() as i32;
        let minutes = (self.current_time % 60.0).floor() as i32;
        let text = format!("{:02}:{:02}", hours, minutes);
        self.clock_label.as_mut().unwrap().set_text(&text);
    }

    pub fn update_orders(&mut self, customer: Gd<Customer>,amount: i32) {
        godot_print!("{} Order received from customer {}", amount, customer.get_name());
    }
}