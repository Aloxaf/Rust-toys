use log::{error, info};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use ulogme::*;

fn main() {
    env_logger::init();

    let cnt = GetKeyPressCnt();

    let mut prev_wm_name = "";
    let mut prev_key_time = SystemTime::now();
    let mut prev_log_name = String::new();
    let (mut key_log_file, mut win_log_file) = get_log_file(&get_log_name());

    // 第一次运行时先写入 __SHUTDOWN, 截断前面的记录和本次记录
    writeln!(&mut win_log_file, "{} __SHUTDOWN", get_timestamp()).unwrap();

    loop {
        // 是否更新日志文件名
        let log_name = get_log_name();
        if log_name != prev_log_name {
            let (f1, f2) = get_log_file(&log_name);
            key_log_file = f1;
            win_log_file = f2;
            prev_log_name = log_name;
        }

        // 窗口切换记录
        if screen_locked() && prev_wm_name != "__LOCKEDSCREEN" {
            prev_wm_name = "__LOCKEDSCREEN";
            info!("{} {}", get_timestamp(), prev_wm_name);
            writeln!(&mut win_log_file, "{} {}", get_timestamp(), prev_wm_name).unwrap();
        } else if let Ok(window) = GetActiveWindow() {
            if window.wm_name != prev_wm_name {
                let log_str = format!(
                    "{} {} \x00 {}",
                    get_timestamp(),
                    window.wm_name,
                    window.wm_class
                );
                info!("{}", log_str);
                writeln!(&mut win_log_file, "{}", log_str).unwrap();

                prev_wm_name = window.wm_name;
            }
        } else {
            error!("No active window");
        }

        // 记录按下的键的次数, 当满10s且按键次数大于0的时候进行一次记录
        if prev_key_time.elapsed().unwrap().as_secs() >= 10 {
            let mut cnt = cnt.lock().unwrap();
            if *cnt != 0 {
                prev_key_time = SystemTime::now();
                info!("{} {}", get_timestamp(), *cnt);
                writeln!(&mut key_log_file, "{} {}", get_timestamp(), *cnt).unwrap();
                *cnt = 0;
            }
        }

        sleep(Duration::from_millis(1000));
    }
}
