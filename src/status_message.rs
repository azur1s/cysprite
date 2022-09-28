pub struct StatusMessage {
    msg: String,
    time: f32,
}

impl StatusMessage {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
            time: 0.0,
        }
    }

    pub fn set(&mut self, msg: &str, time: f32) {
        self.msg = msg.to_string();
        self.time = time;
    }

    pub fn get(&self) -> &str {
        &self.msg
    }

    pub fn update(&mut self, dt: f32) {
        self.time -= dt;
        if self.time < 0.0 {
            self.msg = String::new();
        }
    }
}
