use libc::{c_char, c_ulong};
use std::ffi::CStr;

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
    /// 窗口 PID
    pub pid: u64,
    /// 进程名称
    pub wm_class: &'a str,
    /// 窗口标题
    pub wm_name: &'a str,
}

#[link(name = "X11logger")]
#[link(name = "X11")]
extern "C" {
    fn get_active_window() -> CWindowInfo;
}

/// 以 `\0` 为分隔符分割一个以 `\0\0` 结束的 C 字符串
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

/// 获取当前活动窗口
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
