#[derive(Default)]
pub struct Timer {
    /// whether this should tick
    is_active: bool,
    pub is_loop: bool,
    elasped_ms: f32,
    duration_ms: f32,
}

#[allow(unused)]
impl Timer {
    pub fn new(duration: f32) -> Timer {
        let mut s = Timer::default();
        s.duration_ms = duration;
        return s;
    }

    pub fn tick(&mut self, delta_ms: f32) {
        if !self.is_active {
            return;
        }

        if self.elasped_ms >= self.duration_ms {
            if !self.is_loop {
                self.is_active = false
            }
            self.elasped_ms = 0.0;
            return;
        }

        self.elasped_ms += delta_ms;
    }

    pub fn play(&mut self) {
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
        self.elasped_ms = 0.0;
    }

    // note: this will only be true until the next tick
    // so put is triggered check before the tick
    pub fn is_triggered(&self) -> bool {
        return self.elasped_ms >= self.duration_ms;
    }
}
