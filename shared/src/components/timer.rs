use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Replicate, Component)]
pub struct Timer {
    pub counter: Property<f32>,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new_complete(10.)
    }
}

impl Timer {
    pub fn new(counter: f32) -> Self {
        Self::new_complete(counter)
    }

    pub fn as_string(&self) -> String {
        let str = self.counter.floor().to_string();
        str
    }

    pub fn decr_counter(&mut self) {
        *self.counter -= 1.;
    }

    pub fn incr_counter(&mut self) {
        *self.counter += 1.;
    }
}
