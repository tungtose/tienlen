use naia_bevy_shared::Message;

#[derive(Message, Debug)]
pub struct Counter(pub f32);

impl Counter {
    pub fn new(c: f32) -> Self {
        Self(c)
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    pub fn decr_counter(&mut self) {
        self.0 -= 1.;
    }

    pub fn incr_counter(&mut self) {
        self.0 += 1.;
    }

    pub fn self_check(&mut self) -> bool {
        self.incr_counter();
        // info!("Cur global counter: {}", self.0);

        if self.0 > 60. {
            self.0 = 0.;
            return true;
        }

        false
    }
}
