use time;

use shared::Shared;


struct TimeData {
    start_time: f64,
    last_time: f64,
    current_time: f64,

    fixed: f64,
    fixed_delta: f64,
    scale: f64,

    fps_frame: usize,
    fps_last_time: f64,
    fps: f64,

    delta_time: f64,
    min_delta_time: f64,
    max_delta_time: f64,

    frame: usize,
}

#[derive(Clone)]
pub struct Time {
    data: Shared<TimeData>,
}

impl Time {
    pub fn new() -> Self {
        let start_time = Self::stamp();

        Time {
            data: Shared::new(TimeData {
                start_time: start_time,
                last_time: start_time,
                current_time: start_time,

                fixed: 1f64 / 60f64,
                fixed_delta: 1f64 / 60f64,
                scale: 1f64,

                fps_frame: 0usize,
                fps_last_time: 0f64,
                fps: 60f64,

                delta_time: 1f64 / 60f64,
                min_delta_time: 0.000001f64,
                max_delta_time: 1f64,

                frame: 9usize,
            })
        }
    }

    pub fn update(&mut self) -> &mut Self {
        {
            let ref mut data = self.data;

            data.frame = data.frame + 1;

            data.last_time = data.current_time;
            data.current_time = Self::stamp() - data.start_time;

            data.fps_frame = data.fps_frame + 1;
            if data.fps_last_time + 1f64 < data.current_time {
                data.fps = data.fps_frame as f64 / (data.current_time - data.fps_last_time);
                data.fps_last_time = data.current_time;
                data.fps_frame = 0;
            }

            let delta = (data.current_time - data.last_time) * data.scale;
            data.delta_time = {
                if delta < data.min_delta_time { data.min_delta_time }
                else if delta > data.max_delta_time { data.max_delta_time }
                else { delta }
            };
        }
        self
    }

    pub fn set_scale(&mut self, scale: f64) -> &mut Self {
        {
            let ref mut data = self.data;
            data.scale = scale;
            data.fixed_delta = data.fixed * scale;
        }
        self
    }
    pub fn get_scale(&mut self) -> f64 { self.data.scale }

    pub fn set_fixed_delta(&mut self, fixed_delta: f64) -> &mut Self {
        {
            let ref mut data = self.data;
            data.fixed = fixed_delta;
            data.fixed_delta = data.fixed * data.scale;
        }
        self
    }
    pub fn get_fixed_delta(&mut self) -> f64 { self.data.fixed_delta }

    pub fn get_start_time(&self) -> f64 { self.data.start_time }
    pub fn get_current_time(&self) -> f64 { self.data.current_time }
    pub fn get_delta_time(&self) -> f64 { self.data.delta_time }

    pub fn now(&self) -> f64 { Self::stamp() - self.data.start_time }

    pub fn stamp() -> f64 {
        let current_time = time::get_time();
        (current_time.sec as f64) + (current_time.nsec as f64 / 1000000000f64)
    }
}
