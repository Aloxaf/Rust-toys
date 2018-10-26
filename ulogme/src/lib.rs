extern crate chrono;
extern crate libc;

use chrono::Duration;
use chrono::prelude::*;
use libc::{c_char, c_ulong};
use std::ffi::CStr;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time;
use std::thread::{self, sleep};

pub enum Error {
    NoActiveWindow,
}

#[repr(C)]
struct CWindowInfo {
    pub pid: c_ulong,
    pub wm_class: *const c_char,
    pub wm_name: *const c_char,
}

pub struct WindowInfo<'a> {
    pub pid: u64,
    pub wm_class: &'a str,
    pub wm_name: &'a str,
}

#[link(name = "X11logger")]
#[link(name = "X11")]
extern "C" {
    fn get_active_window() -> CWindowInfo;
}

/// use '\0' as delimiter to split a str terminated with '\0\0'
unsafe fn split_by_nul<'a>(ptr: *const c_char) -> Vec<&'a str> {
    let mut ret = vec![];
    let mut start = ptr;
    for i in 0.. {
        if *ptr.offset(i) == 0 {
            ret.push(CStr::from_ptr(start).to_str().unwrap());
            start = ptr.offset(i + 1);
            if *start == 0 {
                break;
            }
        }
    }
    ret
}

/// get active window
#[allow(non_snake_case)]
pub fn GetActiveWindow<'a>() -> Result<WindowInfo<'a>, Error> {
    let cret = unsafe { get_active_window() };
    // 偶尔会出现找不到激活窗口的情况, 比如打开 telegream 的右键菜单时
    if cret.pid == 0 {
        Err(Error::NoActiveWindow)
    } else {
        Ok(WindowInfo {
            pid: cret.pid,
            // wm_class 含有两项
            wm_class: unsafe { split_by_nul(cret.wm_class)[1] },
            wm_name: unsafe { CStr::from_ptr(cret.wm_name) }.to_str().unwrap(),
        })
    }
}

/// get keyboard devices' id
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

/// 获取按键次数
/// 先获取所有的 keyboard 设备, 然后开一堆 `xinput test $id` 进程, 监测其输出
#[allow(non_snake_case)]
pub fn GetKeyPressCnt() -> Arc<Mutex<i32>> {
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
                .expect("Failed to run `xinput` command");
            let outout = xinput_child.stdout.unwrap();
            let mut bytes = outout.bytes();
            loop {
                // 读取到换行符
                let s = bytes.by_ref()
                    .map(|b| b.unwrap() as char)
                    .take_while(|&c| c != '\n')
                    .collect::<String>();
                if s.find("press").is_some() {
                    let mut cnt = cnt.lock().unwrap();
                    *cnt += 1;
                }
                sleep(time::Duration::from_millis(1000));
            }
        });
        handles.push(handle);
    }

    cnt
}

/// 获取自定义时间戳
/// 早上七点之前算到前一天(肝帝模式
pub fn get_log_name() -> String {
    let dt = Local::now();
    let timestr = dt.format("%Y-%m-%d 07:00:00").to_string();

    let dt = if dt.hour() >= 7 {
        Local.datetime_from_str(&timestr, "%Y-%m-%d %H:%M:%S").unwrap()
    } else {
        let mut dt = Local.datetime_from_str(&timestr, "%Y-%m-%d %H:%M:%S").unwrap();
        dt = dt - Duration::days(1);
        dt
    };

    dt.format("%s").to_string()
}