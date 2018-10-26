use chrono::prelude::*;
use chrono::Duration;
use dbus::{BusType, Connection, Message};
use std::fs::{remove_file, File, OpenOptions};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// 获取自定义时间戳
/// 早上七点之前算到前一天(肝帝模式
pub fn get_log_name() -> String {
    let dt = Local::now();
    let timestr = dt.format("%Y-%m-%d 07:00:00").to_string();

    let dt = if dt.hour() >= 7 {
        Local
            .datetime_from_str(&timestr, "%Y-%m-%d %H:%M:%S")
            .unwrap()
    } else {
        let mut dt = Local
            .datetime_from_str(&timestr, "%Y-%m-%d %H:%M:%S")
            .unwrap();
        dt = dt - Duration::days(1);
        dt
    };

    dt.format("%s").to_string()
}

/// 判断当前是否是锁屏状态
pub fn screen_locked() -> bool {
    let connect = Connection::get_private(BusType::Session).unwrap();
    let mes = Message::new_method_call(
        "org.kde.screensaver",
        "/ScreenSaver",
        "org.freedesktop.ScreenSaver",
        "GetActive",
    )
    .unwrap();
    let ret = connect.send_with_reply_and_block(mes, 2000).unwrap();
    ret.get1().unwrap()
}

/// 创建并覆盖符号链接
pub fn create_symlink(src: &str, dst: &str) {
    if Path::new(dst).exists() {
        remove_file(dst).unwrap();
    }
    symlink(src, dst).unwrap();
}

/// 以追加模式打开文件, 若不存在则创建
pub fn open_file_to_append(path: &str) -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("WTF? Can't open log file to write")
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 创建日志文件
pub fn get_log_file(log_name: &str) -> (File, File) {
    let keylog_name = format!("keyfreq_{}.txt", log_name);
    let winlog_name = format!("window_{}.txt", log_name);
    let key_log_file = open_file_to_append(&keylog_name);
    let win_log_file = open_file_to_append(&winlog_name);
    create_symlink(&keylog_name, "keyfreq_today.txt");
    create_symlink(&winlog_name, "window_today.txt");
    (key_log_file, win_log_file)
}
