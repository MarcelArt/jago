use godot::{classes::Engine, prelude::*};

use crate::singletons::game_data::GameDataSingleton;

// Import modules here
// mod rust_example;
mod selling_phase;
mod customer_spawner;
mod customer;
mod prep_phase;
mod prepare_tab;
mod utils;
mod macros;
mod singletons;

struct GdRust;

#[gdextension]
unsafe impl ExtensionLibrary for GdRust {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                &GameDataSingleton::class_id().to_string_name(), 
                &GameDataSingleton::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let game_data_singleton = &GameDataSingleton::class_id().to_string_name();
    
            if let Some(my_singleton) = engine.get_singleton(game_data_singleton) {
                engine.unregister_singleton(game_data_singleton);
                my_singleton.free();
            }
        }
    }
}
