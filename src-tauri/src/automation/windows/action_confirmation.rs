use std::thread::sleep;
use std::time::{Duration, Instant};

/// Polls a condition at regular intervals until it returns true or the timeout is reached.
/// Returns true if the condition succeeded, false if it timed out.
#[allow(dead_code)]
pub fn wait_for_condition<F>(timeout: Duration, poll_interval: Duration, mut condition: F) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        sleep(poll_interval);
    }
    false
}
