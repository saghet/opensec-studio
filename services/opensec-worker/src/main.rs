mod assessment;
mod rf;
mod risk;
mod wids;

use std::thread;
use std::time::Duration;

fn main() {
    println!(r#"{{"service":"opensec-worker","event":"started","mode":"passive_only"}}"#);
    loop {
        thread::sleep(Duration::from_secs(30));
        println!(r#"{{"service":"opensec-worker","event":"heartbeat"}}"#);
    }
}
