use godot::{classes::{Engine, INode2D, Node2D, RichTextLabel}, prelude::*};

use crate::{customer::Customer, singletons::game_data::GameDataSingleton};

struct CustomerOrder {
    customer: Gd<Customer>,
    amount: i32,
    progress: f32,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SellingPhase {
    base: Base<Node2D>,
    clock_label: Option<Gd<RichTextLabel>>,
    money_label: Option<Gd<RichTextLabel>>,
    current_time: f64,
    end_time: f64,  // End time in minutes (e.g., 20:00 = 1200 minutes)
    is_day_over: bool,
    time_speed: f64, // Time speed multiplier
    serving_speed: f32,
    orders: Vec<CustomerOrder>,
    
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
            money_label: None,
            current_time: (8 * 60) as f64, // Start at 8:00 AM
            time_speed: 1 as f64, // Normal speed
            end_time: (17 * 60) as f64, // End at 8:00 PM
            is_day_over: false,
            time_multiplier: 5 as f64, // Default time multiplier
            serving_speed: 1.0, // Default serving speed
            orders: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let game_data = GameDataSingleton::get_instance();
        self.clock_label = Some(self.base().get_node_as("UI/VBoxContainer/ClockLabel"));
        
        self.money_label = Some(self.base().get_node_as("UI/VBoxContainer/MoneyLabel"));
        let text = format!("K. {}", game_data.bind().money);
        self.money_label.as_mut().unwrap().set_text(&text);
    }

    fn process(&mut self, _delta: f64) {
        self.progress_time(_delta);
        self.serve_customer(_delta);
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

    pub fn update_orders(&mut self, mut customer: Gd<Customer>, amount: i32) {
        let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();
        let mut game_data = game_data.bind_mut();
        
        if game_data.stock < amount {
            customer.bind_mut().complete_order(false);
            return;
        }
        game_data.stock -= amount;
        self.orders.push(CustomerOrder { customer, amount, progress: 0.0 });
        godot_print!("Stock: {} -> {}", game_data.stock + amount, game_data.stock);
    }

    fn serve_customer(&mut self, delta: f64) {
        let mut paid_amount = 0;
        
        // mutable borrow self
        {
            let game_data = GameDataSingleton::get_instance();
            let order = self.orders.get_mut(0);
            if order.is_none() {
                return;
            }
            let order = order.unwrap();
            
            let progress = delta as f32 * self.serving_speed;
            order.progress += progress;
            if order.progress >= order.amount as f32 { // order complete remove order queue and change customer state
                godot_print!("Served customer {}", order.customer.get_name());
                order.customer.bind_mut().complete_order(true);
                paid_amount = order.amount * game_data.bind().price;
                self.orders.remove(0);
            }
        }
        
        self.getting_paid(paid_amount);
    }

    fn getting_paid(&mut self, amount: i32) {
        let mut game_data = GameDataSingleton::get_instance();
        let money = game_data.bind_mut().add_money(amount);
        let text = format!("Rp. {}", money);
        self.money_label.as_mut().unwrap().set_text(&text);
    }
}