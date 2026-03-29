use std::time::Duration;

use crate::prelude::*;

impl<'a> Context<'a> {
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta_time().as_secs_f32()
    }
}
