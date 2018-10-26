use std::io::prelude::*;
use std::io::{BufRead, Cursor};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

/// 获取所有键盘设备的 id
fn get_device_list() -> Vec<i32> {
    let mut devices = vec![];

    let xinput = Command::new("xinput").output().unwrap();
    let output = String::from_utf8(xinput.stdout).unwrap();
    let device_list = output.split('\n').collect::<Vec<_>>();
    for device in device_list {
        if device.find("slave  keyboard").is_some() {
            let pos = device.find("id=").unwrap(); // return the position of '='
            let id = device
                .chars()
                .skip(pos + 1)
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            let id = id.parse::<i32>().unwrap();
            devices.push(id);
        }
    }

    devices
}

/// 获取修饰键的键码
fn get_modifier_keys() -> Vec<String> {
    let mut ids = vec![];
    let cmd_xmodmap = Command::new("xmodmap")
        .arg("-pk")
        .stdout(Stdio::piped())
        .spawn()
        .expect("WTF? No xmodmap ??");
    let cmd_grep = Command::new("grep")
        .arg("-iE")
        .arg(r#"\b(shift_[lr]|alt_[lr]|control_[lr]|caps_lock)\b"#)
        .stdin(cmd_xmodmap.stdout.unwrap())
        .output()
        .expect("WFT? No grep ??");
    let cursor = Cursor::new(cmd_grep.stdout);
    for line in cursor.lines() {
        let line = line.unwrap();
        let mut fields = line.split_whitespace();
        let id = fields.next().unwrap().to_string();
        ids.push(id);
    }
    ids
}

/// 获取按键次数
/// 先获取所有的 keyboard 设备, 然后开一堆 `xinput test $id` 进程, 监测其输出
#[allow(non_snake_case)]
pub fn GetKeyPressCnt() -> Arc<Mutex<i32>> {
    let cnt = Arc::new(Mutex::new(0));
    let modifier_keys = Arc::new(get_modifier_keys());

    let devices = get_device_list();
    let mut handles = vec![];

    for device in devices {
        let cnt = Arc::clone(&cnt);
        let modifier_keys = Arc::clone(&modifier_keys);

        let handle = thread::spawn(move || {
            let xinput_child = Command::new("xinput")
                .arg("test")
                .arg(device.to_string())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to run `xinput` command");
            let outout = xinput_child.stdout.unwrap();
            let mut bytes = outout.bytes();

            loop {
                // 读取到换行符
                let s = bytes
                    .by_ref()
                    .map(|b| b.unwrap() as char)
                    .take_while(|&c| c != '\n')
                    .collect::<String>();
                let s = s.split_whitespace().collect::<Vec<&str>>();
                if s.len() >= 3 && s[1] == "press" && !modifier_keys.contains(&s[2].to_string()) {
                    let mut cnt = cnt.lock().unwrap();
                    *cnt += 1;
                }
            }
        });
        handles.push(handle);
    }

    cnt
}
