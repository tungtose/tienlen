use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Replicate, Component)]
pub struct Counter {
    pub counter: Property<f32>,
}

const TIMEOUT: f32 = 10.;

impl Default for Counter {
    fn default() -> Self {
        Self::new_complete(TIMEOUT)
    }
}

impl Counter {
    pub fn new(counter: f32) -> Self {
        Self::new_complete(counter)
    }

    pub fn check_over(&mut self) -> bool {
        if *self.counter < 0. {
            *self.counter = TIMEOUT;
            return true;
        }

        false
    }

    pub fn recount(&mut self) {
        *self.counter = TIMEOUT;
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