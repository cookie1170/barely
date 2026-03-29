use std::time::Duration;

use crate::prelude::*;

impl Context<'_> {
    #[must_use]
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    #[must_use]
    pub fn delta_secs(&self) -> f32 {
        self.delta_time().as_secs_f32()
    }
}
