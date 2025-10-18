use godot::{classes::{Button, INode2D, Node2D, RichTextLabel}, prelude::*};

use crate::{customer::Customer, enums::customer_feedback::CustomerFeedback, singletons::game_data::GameDataSingleton};

struct CustomerOrder {
    customer: Gd<Customer>,
    amount: i32,
    progress: f32,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SellingPhase {
    base: Base<Node2D>,
    current_time: f64,
    end_time: f64,  // End time in minutes (e.g., 20:00 = 1200 minutes)
    is_day_over: bool,
    time_speed: f64, // Time speed multiplier
    serving_speed: f32,
    orders: Vec<CustomerOrder>,
    love_count: i32,
    like_count: i32,
    dislike_count: i32,
    
    // Change or add your own properties here
    #[export]
    time_multiplier: f64,
    #[export]
    clock_label: Option<Gd<RichTextLabel>>,
    #[export]
    money_label: Option<Gd<RichTextLabel>>,
    #[export]
    love_label: Option<Gd<RichTextLabel>>,
    #[export]
    like_label: Option<Gd<RichTextLabel>>,
    #[export]
    dislike_label: Option<Gd<RichTextLabel>>,
    #[export]
    day_label: Option<Gd<RichTextLabel>>,
    #[export]
    stock_label: Option<Gd<RichTextLabel>>,
    #[export]
    skip_button: Option<Gd<Button>>,
}

#[godot_api]
impl INode2D for SellingPhase {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            clock_label: None,
            money_label: None,
            love_label: None,
            like_label: None,
            dislike_label: None,
            day_label: None,
            stock_label: None,
            skip_button: None,
            current_time: (8 * 60) as f64, // Start at 8:00 AM
            time_speed: 1 as f64, // Normal speed
            end_time: (17 * 60) as f64, // End at 8:00 PM
            is_day_over: false,
            time_multiplier: 5 as f64, // Default time multiplier
            serving_speed: 1.0, // Default serving speed
            orders: Vec::new(),
            love_count: 0,
            like_count: 0,
            dislike_count: 0,
        }
    }

    fn ready(&mut self) {
        let game_data = GameDataSingleton::get_instance();

        let text = format!("{}", game_data.bind().money);
        self.money_label.as_mut().unwrap().set_text(&text);

        self.get_day_label().unwrap().set_text(&format!("Day {}", game_data.bind().day));

        self.get_stock_label().unwrap().set_text(&format!("Stock: {}", game_data.bind().stock));

        self.get_skip_button().unwrap()
            .signals()
            .pressed()
            .connect_other(&*self, Self::end_day);
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
        self.update_clock_label();
    }

    fn update_clock_label(&mut self) {
        let hours = (self.current_time / 60.0).floor() as i32;
        let minutes = (self.current_time % 60.0).floor() as i32;
        let text = format!("{:02}:{:02}", hours, minutes);
        self.clock_label.as_mut().unwrap().set_text(&text);
    }

    pub fn update_orders(&mut self, mut customer: Gd<Customer>, amount: i32) {
        let mut game_data = GameDataSingleton::get_instance();
        // let mut game_data = game_data.bind_mut();
        
        if game_data.bind().stock < amount {
            customer.bind_mut().complete_order(false);
            return;
        }
        game_data.bind_mut().stock -= amount;
        self.orders.push(CustomerOrder { customer, amount, progress: 0.0 });
        godot_print!("Stock: {} -> {}", game_data.bind().stock + amount, game_data.bind().stock);
        self.get_stock_label().unwrap().set_text(&format!("Stock: {}", game_data.bind().stock));
    }

    fn serve_customer(&mut self, delta: f64) {
        let mut paid_amount = 0;
        let mut feedback = CustomerFeedback::None;
        
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

                feedback = order.customer.bind_mut().complete_order(true);
                
                paid_amount = order.amount * game_data.bind().price;
                self.orders.remove(0);
            }
        }
        
        self.increase_counter(feedback);
        self.getting_paid(paid_amount);
    }

    fn getting_paid(&mut self, amount: i32) {
        let mut game_data = GameDataSingleton::get_instance();
        let money = game_data.bind_mut().add_money(amount);
        let text = format!("{}", money);
        self.money_label.as_mut().unwrap().set_text(&text);
    }

    fn increase_counter(&mut self, feedback: CustomerFeedback) {
        match  feedback {
            CustomerFeedback::Love => self.increase_love_count(),
            CustomerFeedback::Like => self.increase_like_count(),
            CustomerFeedback::Dislike => self.increase_dislike_count(),
            _ => (),
        }
    }

    fn increase_love_count(&mut self) {
        self.love_count += 1;
        self.get_love_label().unwrap().set_text(&format!("{}", self.love_count));
    }

    fn increase_like_count(&mut self) {
        self.like_count += 1;
        self.get_like_label().unwrap().set_text(&format!("{}", self.like_count));
    }

    fn increase_dislike_count(&mut self) {
        self.dislike_count += 1;
        self.get_dislike_label().unwrap().set_text(&format!("{}", self.dislike_count));
    }

    fn end_day(&mut self) {
        let mut game_data = GameDataSingleton::get_instance();
        game_data.bind_mut().day += 1;
        game_data.bind_mut().stock = 0;

        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/prep_phase.tscn");
    }
}