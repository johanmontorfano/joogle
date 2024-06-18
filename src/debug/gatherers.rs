use std::time::{SystemTime, UNIX_EPOCH};

/// Get progression data of a system to show timing data.
pub struct TimingGatherer {
    start_time: u128,
    pub actions_done: u128
}

impl TimingGatherer {
    pub fn init() -> Self {
        Self { start_time: 0, actions_done: 0 }
    }

    /// Delete previously recorded data and creates a new timing session.
    pub fn start_gathering(&mut self) {
        self.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }

    /// Log a new action as done.
    pub fn action_done(&mut self) {
        self.actions_done += 1;
    }

    /// Log data.
    pub fn log_gathered_data(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let duration = now - self.start_time;
        let avg_s_process_time = duration as f64 / 
            self.actions_done as f64 / 
            1000.0;

        println!(
            "Total duration: {}s | Total units: {}\nAvg. process time: {}s",
            duration / 1000, self.actions_done, avg_s_process_time
        );
    }
}
