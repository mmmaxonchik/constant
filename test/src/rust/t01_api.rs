use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let _pid = std::process::id();
    let _ = fs::read_to_string("/proc/self/stat").unwrap_or_default();
    let _ = fs::read("/proc/self/cmdline").unwrap_or_default();
    thread::sleep(Duration::from_millis(13));
}
