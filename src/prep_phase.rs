use godot::{classes::{Button, INode2D, Node2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct PrepPhase {
    base: Base<Node2D>,
    start_day_button: Option<Gd<Button>>,

    // Change or add your own properties here
    // #[export]
}

#[godot_api]
impl INode2D for PrepPhase {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            start_day_button: None,
        }
    }

    fn ready(&mut self) {
        self.start_day_button = Some(self.base().get_node_as("UI/StartDayButton"));
        let start_day_button = self.start_day_button.as_ref().unwrap();
        start_day_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::_on_start_day_button_pressed);
    }

    fn process(&mut self, _delta: f64) {

    } 
}     

impl PrepPhase {
    fn _on_start_day_button_pressed(&mut self) {
        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/selling_phase.tscn");
    }
}