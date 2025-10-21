use godot::{classes::{Button, Control, IControl, Panel}, prelude::*};

use crate::singletons::game_data::GameDataSingleton;

#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu {
    base: Base<Control>,

    // Change or add your own properties here
    #[export]
    continue_button: Option<Gd<Button>>,
    #[export]
    new_game_button: Option<Gd<Button>>,
    #[export]
    credit_button: Option<Gd<Button>>,
    #[export]
    exit_button: Option<Gd<Button>>,
    #[export]
    close_credit_button: Option<Gd<Button>>,
    #[export]
    credit_panel: Option<Gd<Panel>>,
}

#[godot_api]
impl IControl for MainMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            continue_button: None,
            new_game_button: None,
            credit_button: None,
            close_credit_button: None,
            exit_button: None,
            credit_panel: None,
        }
    }

    fn ready(&mut self) {
        let game_data = GameDataSingleton::get_instance();
        
        self.get_continue_button().unwrap().set_visible(false);
        self.get_credit_panel().unwrap().set_visible(false);
        
        if game_data.bind().is_save_exist() {
            self.get_continue_button().unwrap().set_visible(true);
        }

        let new_game_button = self.get_new_game_button().unwrap();
        new_game_button.signals()
            .pressed()
            .connect_other(&*self, Self::new_game);

        let exit_button = self.get_exit_button().unwrap();
        exit_button.signals()
            .pressed()
            .connect_other(&*self, Self::exit_game);

        let credit_button = self.get_credit_button().unwrap();
        credit_button.signals()
            .pressed()
            .connect_other(&*self, Self::open_credit);

        let close_credit_button = self.get_close_credit_button().unwrap();
        close_credit_button.signals()
            .pressed()
            .connect_other(&*self, Self::close_credit);

        let continue_button = self.get_continue_button().unwrap();
        continue_button.signals()
            .pressed()
            .connect_other(&*self, Self::load_game);
    }

    fn process(&mut self, _delta: f64) {
    }  
}

impl MainMenu {
    fn new_game(&mut self) {
        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/prep_phase.tscn");
    }

    fn exit_game(&mut self) {
        self.base().get_tree().unwrap().quit();
    }

    fn open_credit(&mut self) {
        self.get_credit_panel().unwrap().set_visible(true);
    }

    fn close_credit(&mut self) {
        self.get_credit_panel().unwrap().set_visible(false);
    }

    fn load_game(&mut self) {
        let mut game_data = GameDataSingleton::get_instance();
        game_data.bind_mut().load_game();

        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/prep_phase.tscn");
    }
}
        