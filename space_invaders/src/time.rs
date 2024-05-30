use core::ops::Sub;
use core::time::Duration;

pub trait TimeManagerInterface {
    /// a monotonically increasing clock.
    fn now(&self) -> Duration;

    /// how much time passed since `time_in_the_past`.
    fn since(&self, time_in_the_past: Duration) -> Duration {
        self.now().sub(time_in_the_past)
    }
}
