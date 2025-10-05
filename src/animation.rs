pub trait AnimatedValue {
    /// Given the increase in time, move the current value a bit towards the target
    fn tick(&mut self, delta_ms: f32) -> Result<(), anyhow::Error>;
    fn pause(&mut self);
    fn play(&mut self);
    fn stop(&mut self);
    fn restart(&mut self);
    /// The percentage of completion btw 0 & 1
    fn percentage(&self) -> f32;
    /// Set current value to when it's % btw 0 & 1 complete
    fn seek(&mut self, percentage: f32) -> Result<(), anyhow::Error>;
}

pub struct AnimatedNumber {
    pub target_value: f32,
    pub initial_value: f32,
    pub duration_ms: f32, // duration in milliseconds
    current_value: f32,
    is_playing: bool,
    pub is_loop: bool,
}

impl AnimatedValue for AnimatedNumber {
    // move the current value towards the target val based on the time
    fn tick(&mut self, delta_ms: f32) -> Result<(), anyhow::Error> {
        if self.duration_ms == 0.0 || !self.is_playing {
            return Ok(());
        }

        // for this, we'll check how much time has passed with the percentage
        // ex: if it's a 2 sec anim, and we're 50% in, that means we've used (2 * 0.5) seconds
        // now we add the tick, that'll be (2 * 0.5) + delta, then we can check time progress
        // ie. (elasped_time/duration_ms) then we seek to that position
        let elasped_time = self.duration_ms * self.percentage(); // how long has passed already
        let ticked_time = elasped_time + delta_ms;
        let mut progress_in_time = ticked_time / self.duration_ms;

        // it's possible that we can exceed the duration, depending on the delta
        // when this happens we should just cap it
        if progress_in_time >= 1.0 {
            print!("Got progress in time {progress_in_time}. ");
            progress_in_time = 1.0;

            // seek here before restart
            self.seek(progress_in_time)?;

            // if it's a loop restart
            if self.is_loop {
                self.restart();
            }
        } else {
            self.seek(progress_in_time)?;
        }
        return Ok(());
    }

    fn pause(&mut self) {
        self.is_playing = false;
    }

    fn play(&mut self) {
        // if the current value is not the target value then play
        self.is_playing = self.current_value != self.target_value;
        if !self.is_playing {
            // warn
            println!("warn: Attempting to play finished animation")
        }
    }

    fn percentage(&self) -> f32 {
        let current_diff = self.target_value - self.current_value;
        let full_diff = self.target_value - self.initial_value;

        return current_diff / full_diff;
    }

    fn stop(&mut self) {
        self.is_playing = false;
        self.current_value = self.initial_value;
    }

    fn restart(&mut self) {
        self.stop();
        self.play();
    }

    fn seek(&mut self, percentage: f32) -> Result<(), anyhow::Error> {
        if percentage < 0.0 || percentage > 1.0 {
            anyhow::bail!("seek percentage out of bounds")
        }

        // we'll get this by doing
        // (target-initial) * percent ; where percent is btw 0 & 1
        // then add the result to initial
        let diff = (self.target_value - self.initial_value) * percentage;
        self.current_value = diff + self.initial_value;
        return Ok(());
    }
}
