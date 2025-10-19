use godot::{classes::{Button, Control, IControl, LineEdit, RichTextLabel}, prelude::*};

use crate::{error_alert::ErrorAlert, get_node_by_abs_path, singletons::game_data::GameDataSingleton};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ShopTab {
    base: Base<Control>,
    error_alert: Option<Gd<ErrorAlert>>,

    // Change or add your own properties here
    #[export]
    coffee_input: Option<Gd<LineEdit>>,
    #[export]
    milk_input: Option<Gd<LineEdit>>,
    #[export]
    sugar_input: Option<Gd<LineEdit>>,
    #[export]
    cup_input: Option<Gd<LineEdit>>,
    #[export]
    money_label: Option<Gd<RichTextLabel>>,
    #[export]
    buy_button: Option<Gd<Button>>,
}

#[godot_api]
impl IControl for ShopTab {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            coffee_input: None,
            milk_input: None,
            sugar_input: None,
            cup_input: None,
            money_label: None,
            buy_button: None,
            error_alert: None,
        }
    }

    fn ready(&mut self) {
        let coffee_input = self.get_coffee_input().unwrap();
        coffee_input.signals()
            .text_changed()
            .connect_other(&*self, Self::on_change_buy_amount);

        let milk_input = self.get_milk_input().unwrap();
        milk_input.signals()
            .text_changed()
            .connect_other(&*self, Self::on_change_buy_amount);

        let sugar_input = self.get_sugar_input().unwrap();
        sugar_input.signals()
            .text_changed()
            .connect_other(&*self, Self::on_change_buy_amount);

        let cup_input = self.get_cup_input().unwrap();
        cup_input.signals()
            .text_changed()
            .connect_other(&*self, Self::on_change_buy_amount);

        let buy_button = self.get_buy_button().unwrap();
        buy_button.signals()
            .pressed()
            .connect_other(&*self, Self::on_buy);

        self.error_alert = Some(get_node_by_abs_path!(self.base(), "PrepPhase/ErrorAlert"));
    }

    fn process(&mut self, _delta: f64) {
    } 
}     

#[godot_api]
impl ShopTab {
    #[signal]
    pub fn on_buy_success();

    fn on_change_buy_amount(&mut self, _: GString) {
        self.sanitize_input();
        let coffee = self.get_coffee_input().unwrap().get_text().to_int() as i32;
        let milk = self.get_milk_input().unwrap().get_text().to_int() as i32;
        let sugar = self.get_sugar_input().unwrap().get_text().to_int() as i32;
        let cup = self.get_cup_input().unwrap().get_text().to_int() as i32;

        let coffee_subtotal = coffee * 120;
        let milk_subtotal = milk * 30;
        let sugar_subtotal = sugar * 20;
        let cup_subtotal = cup * 50;

        let total = coffee_subtotal + milk_subtotal + sugar_subtotal + cup_subtotal;
        self.get_money_label().unwrap().set_text(&format!("{}", total));
    }

    fn on_buy(&mut self) {
        let mut game_data = GameDataSingleton::get_instance();
        let mut money_label = self.get_money_label().unwrap();
        let total = money_label.get_text().to_int() as i32;
        if game_data.bind().money < total {
            let error_alert = self.error_alert.as_mut().unwrap();
            error_alert.bind_mut().show_alert(GString::from("Not enough money"));
            return;
        }

        // Mutate game data singleton
        let coffee = self.get_coffee_input().unwrap().get_text().to_float() as f32;
        let milk = self.get_milk_input().unwrap().get_text().to_float() as f32;
        let sugar = self.get_sugar_input().unwrap().get_text().to_float() as f32;
        let cup = self.get_cup_input().unwrap().get_text().to_int() as i32;

        let coffee = coffee * 300.0;
        let milk = milk * 1000.0;
        let sugar = sugar * 1000.0;
        let cup = cup * 50;

        game_data.bind_mut().add_money(-total);
        game_data.bind_mut().inventory.coffee += coffee;
        game_data.bind_mut().inventory.milk += milk;
        game_data.bind_mut().inventory.sugar += sugar;
        game_data.bind_mut().cup += cup;

        // Reset ui
        self.get_coffee_input().unwrap().set_text("0");
        self.get_milk_input().unwrap().set_text("0");
        self.get_sugar_input().unwrap().set_text("0");
        self.get_cup_input().unwrap().set_text("0");
        money_label.set_text("0");

        self.signals().on_buy_success().emit();
    }

    fn sanitize_input(&mut self) {
        let coffee = self.get_coffee_input().unwrap().get_text().to_int() as i32;
        let milk = self.get_milk_input().unwrap().get_text().to_int() as i32;
        let sugar = self.get_sugar_input().unwrap().get_text().to_int() as i32;
        let cup = self.get_cup_input().unwrap().get_text().to_int() as i32;

        if coffee < 0 {
            self.get_coffee_input().unwrap().set_text("0");
        }
        if milk < 0 {
            self.get_milk_input().unwrap().set_text("0");
        }
        if sugar < 0 {
            self.get_sugar_input().unwrap().set_text("0");
        }
        if cup < 0 {
            self.get_cup_input().unwrap().set_text("0");
        }
    }
}