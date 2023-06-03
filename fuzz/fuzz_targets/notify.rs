use honggfuzz::fuzz;
use penrose::util::notify;
use std::process::Command;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(msg) = std::str::from_utf8(data) {
                // Call the notify function with the fuzzed input
                let _ = notify(msg);

            }
        });
    }
}
