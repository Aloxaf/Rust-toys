extern crate chrono;
extern crate libc;

use chrono::Duration;
use chrono::prelude::*;
use libc::{c_char, c_ulong};
use std::ffi::CStr;
use std::process::Command;

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
pub fn get_device_list() -> Vec<i32> {
    let mut devices = vec![];

    let xinput = Command::new("xinput").output().unwrap();
    let output = String::from_utf8(xinput.stdout).unwrap();
    let device_list = output.split('\n').collect::<Vec<_>>();
    for device in device_list {
        if let Some(_) = device.find("slave  keyboard") {
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