#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod system_set;
        mod cards;
        mod welcome;
        mod resources;
        mod systems;
        mod ui;
        mod app;
        mod components;
        mod states;
        mod assets;
        mod game;

        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            app::run();

            Ok(())
        }
    }
}
