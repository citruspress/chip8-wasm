pub struct GameTime {
    elapsed: f64,
    time: f64,
}

impl GameTime {
    pub fn new(current_time: f64) -> GameTime {
        GameTime {
            elapsed: 0f64,
            time: current_time,
        }
    }

    pub fn update(&mut self, current_time: f64) {
        self.elapsed = current_time - self.time;
        self.time = current_time;
    }

    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }
}