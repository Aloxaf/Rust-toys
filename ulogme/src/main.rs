use std::fs::{OpenOptions, File, remove_file};
use std::io::prelude::*;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use ulogme::{GetActiveWindow, GetKeyPressCnt, get_log_name};

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// 创建并覆盖符号链接
fn create_symlink(src: &str, dst: &str) {
    if Path::new(dst).exists() {
        remove_file(dst).unwrap();
    }
    symlink(src, dst).unwrap();
}

/// 创建日志文件
fn get_log_file(log_name: &str) -> (File, File) {
    let keylog_name = format!("keyfreq_{}.txt", log_name);
    let winlog_name = format!("window_{}.txt", log_name);
    let key_log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&keylog_name)
        .expect("WTF? Can't open log file to write");
    let win_log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&winlog_name)
        .expect("WTF? Can't open log file to write");
    create_symlink(&keylog_name, "keyfreq_today.txt");
    create_symlink(&winlog_name, "window_today.txt");
    (key_log_file, win_log_file)
}

fn main() {
    let cnt = GetKeyPressCnt();

    let mut prev_wm_name = "";
    let mut prev_key_time = SystemTime::now();
    let mut prev_win_time = SystemTime::now();
    let mut prev_log_name = String::new();

    let (mut key_log_file, mut win_log_file) = get_log_file(&get_log_name());

    loop {
        let log_name = get_log_name();
        if log_name != prev_log_name {
            let (f1, f2) = get_log_file(&log_name);
            key_log_file = f1;
            win_log_file = f2;
            prev_log_name = log_name;
        }

        // 窗口切换记录
        if let Ok(window) = GetActiveWindow() {
            if window.wm_name != prev_wm_name {
                // FIXME: telegram 右键菜单也会导致没有记录
                // 如果上次记录的时间到现在已经过了超过 30s, 则判定在这段时间内属于休眠时间
                if prev_win_time.elapsed().unwrap().as_secs() >= 30 {
                    prev_wm_name = "__LOCKEDSCREEN";
                    println!("{} {}", get_timestamp(), prev_wm_name);
                    writeln!(&mut win_log_file, "{} {}", get_timestamp(), prev_wm_name);
                } else {
                    prev_wm_name = window.wm_name;
                    println!("{} {} \x00 {}", get_timestamp(), window.wm_name, window.wm_class);
                    writeln!(&mut win_log_file, "{} {} \x00 {}", get_timestamp(), window.wm_name, window.wm_class);
                }
            }
            prev_win_time = SystemTime::now();
        }

        // 记录按下的键的次数, 当满10s且按键次数大于0的时候进行一次记录
        if prev_key_time.elapsed().unwrap().as_secs() >= 10 {
            let mut cnt = cnt.lock().unwrap();
            if *cnt != 0 {
                prev_key_time = SystemTime::now();
                println!("{} {}", get_timestamp(), *cnt);
                writeln!(&mut key_log_file, "{} {}", get_timestamp(), *cnt);
                *cnt = 0;
            }
        }

        sleep(Duration::from_millis(1000));
    }
}
