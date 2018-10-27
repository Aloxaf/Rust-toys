extern crate chrono;
extern crate dbus;
extern crate env_logger;
extern crate libc;
#[macro_use]
extern crate log;
extern crate regex;

mod keyboard;
mod utils;
mod window;

pub use self::keyboard::GetKeyPressCnt;
pub use self::utils::*;
pub use self::window::GetActiveWindow;
