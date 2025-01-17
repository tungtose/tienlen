#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        mod system_set;
        mod resources;
        mod systems;
        mod ui;
        mod app;
        mod components;
        mod states;
        mod assets;
        mod fps;
        mod game;
        mod welcome;

        fn main() {
            app::run();
        }
    }
}
