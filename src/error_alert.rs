use godot::{classes::{Button, Control, IControl, RichTextLabel}, prelude::*};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ErrorAlert {
    base: Base<Control>,

    // Change or add your own properties here
    #[export]
    message_label: Option<Gd<RichTextLabel>>,
    #[export]
    ok_button: Option<Gd<Button>>,
}

#[godot_api]
impl IControl for ErrorAlert {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            message_label: None,
            ok_button: None,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_visible(false);
        
        let ok_button = self.get_ok_button().unwrap();
        ok_button.signals()
            .pressed()
            .connect_other(&*self, Self::close_alert);
    }

    fn process(&mut self, _delta: f64) {
    }
}

impl ErrorAlert {
    fn close_alert(&mut self) {
        self.base_mut().set_visible(false);
    }

    pub fn show_alert(&mut self, message: GString) {
        self.get_message_label().unwrap().set_text(&message);
        self.base_mut().set_visible(true);
    }
}
        