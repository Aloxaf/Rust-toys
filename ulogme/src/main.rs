use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use ulogme::{GetActiveWindow, get_device_list, get_log_name};

fn main() {
    let mut prev_wm_name = "";
    let cnt = Arc::new(Mutex::new(0));

    let devices = get_device_list();
    let mut handles = vec![];

    for device in devices {
        let cnt = Arc::clone(&cnt);
        let handle = thread::spawn(move || {
            let xinput_child = Command::new("xinput")
                .arg("test")
                .arg(device.to_string())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to run `input` command");
            let outout = xinput_child.stdout.unwrap();
            let mut bytes = outout.bytes();
            loop {
                let s = bytes.by_ref()
                    .map(|b| b.unwrap() as char)
                    .take_while(|&c| c != '\n')
                    .collect::<String>();
                if let Some(_) = s.find("press") {
                    let mut cnt = cnt.lock().unwrap();
                    *cnt += 1;
                }
                sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    let mut prev_time = Instant::now();

    println!("log: {}", get_log_name());

    loop {
        if let Ok(window) = GetActiveWindow() {
            if window.wm_name != prev_wm_name {
                prev_wm_name = window.wm_name;
                println!("{} {}", window.wm_class, window.wm_name);

            }
        }
        sleep(Duration::from_millis(500));
        if prev_time.elapsed().as_secs() >= 10 {
            prev_time = Instant::now();
            let mut cnt = cnt.lock().unwrap();
            println!("{}", *cnt);
            *cnt = 0;
        }
    }
}
