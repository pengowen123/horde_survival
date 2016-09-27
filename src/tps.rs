use consts::misc::TPS;

use std::time::{Instant, Duration};

pub struct Ticks {
    expected_elapsed: Duration,
    sleeping_until: Instant,
    time_0: Instant,
    time_1: Instant,
    time_2: Instant,
    frame_time: Duration,
    update_time: Duration,
}

impl Ticks {
    pub fn new() -> Ticks {
        let now = Instant::now();
        let zero = Duration::from_millis(0);

        Ticks {
            expected_elapsed: Duration::from_millis(1_000_000_000 / TPS / 1_000_000),
            sleeping_until: now,
            time_0: now,
            time_1: now,
            time_2: now,
            frame_time: zero,
            update_time: zero,
        }
    }

    pub fn begin_tick(&mut self) {
        self.time_0 = Instant::now();
    }

    pub fn measure_frame_1(&mut self) {
        self.time_1 = Instant::now();
        self.frame_time = self.time_1 - self.time_0;
    }

    pub fn measure_frame_2(&mut self) {
        self.frame_time += Instant::now() - self.time_2;
    }

    pub fn measure_update(&mut self) {
        self.time_2 = Instant::now();
        self.update_time = self.time_2 - self.time_1;
    }

    pub fn is_sleeping(&self) -> bool {
        Instant::now() < self.sleeping_until
    }

    pub fn get_debug_info(&self) -> [Duration; 3] {
        [self.frame_time, self.update_time, self.frame_time + self.update_time]
    }
   
    pub fn end_tick(&mut self) {
        let current = Instant::now();
        let elapsed = current - self.time_0;

        if elapsed < self.expected_elapsed {
            self.sleeping_until = current + (self.expected_elapsed - elapsed);
        }
    }
}
