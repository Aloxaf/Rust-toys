use std::time::Duration;
use std::thread::sleep;
use ulogme::GetActiveWindow;

fn main() {
    let mut prev_wm_name = "";
    loop {
        if let Ok(window) = GetActiveWindow() {
            if window.wm_name != prev_wm_name {
                prev_wm_name = window.wm_name;
                println!("{} {}", window.wm_class, window.wm_name);
            }
        }
        sleep(Duration::from_millis(500));
    }
}
