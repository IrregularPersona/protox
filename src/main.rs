#![cfg_attr(feature = "packaged", windows_subsystem = "windows")]
use std::{env, sync::{LazyLock, Mutex}};
use windows::Win32::UI::WindowsAndMessaging::*;

mod app;
mod config;
mod constants;
mod handlers;
mod utils;
mod window;
pub mod modules {
    pub mod blocklist;
    pub mod css_override;
    pub mod flaglist;
    pub mod obs;
    pub mod priority;
    pub mod swapper;
    pub mod userscripts;
}

static LAUNCH_ARGS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(env::args().skip(1).collect()));
static CONFIG: LazyLock<Mutex<config::Config>> = LazyLock::new(|| Mutex::new(config::Config::load()));

fn main() {
    if modules::obs::handle_cli_flags() {
        return;
    }

    utils::register_instance();
    #[cfg(feature = "packaged")]
    {
        utils::set_panic_hook().ok();
        utils::installer_cleanup().ok();
    }

    if let Err(e) = app::init_fs() {
        eprintln!("failed to set all the files in place {}", e);
    }

    let _window = app::create_main_window(None);
    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0).into() } {
        unsafe {
            _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    CONFIG.lock().unwrap().save();
}
