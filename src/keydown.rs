

use std::{sync::Arc, ptr::{null_mut, null, NonNull}};
use once_cell::sync::Lazy;


use parking_lot::Mutex; 
 
use crate::hotkey::Key;

use super::hotkey;

fn format_json_error<T: std::fmt::Display>(err: T) {
    "error";
}

static HOTKEY_MANAGER: Lazy<Arc<Mutex<hotkey::HotkeyManager>>> = 
Lazy::new(|| Arc::new(Mutex::new(hotkey::HotkeyManager::new())));
// Lazy::new(Arc::new(Mutex::new(hotkey::HotkeyManager::new())));
 
static mut CALLBACK_HODLER: Option<extern fn(i32)> = Option::None;
static mut CALLBACK_RUST_HODLER: Option<fn(i32)> = Option::None;

// static mut CALLBACK_INFO_LIST: Arr
struct KeyEventInfo {
    key: String,
    callback_id: i32
}

static mut CALLBACK_INFO_LIST: Vec<KeyEventInfo> = Vec::new();
static mut IS_LOG_OPEN: bool = false;

pub fn set_log_status(is_open: bool) {
    unsafe {
        IS_LOG_OPEN = is_open;
    }
}
 

pub fn _init_invoke_callback(func: extern fn(i32)) {
    // println!("_init_invoke_callback ");
    unsafe {
        CALLBACK_HODLER = Option::Some(func);
    }
}

pub fn _init_invoke_rust_callback(func: fn(i32)) {
    // println!("_init_invoke_rust_callback");
    unsafe {
        CALLBACK_RUST_HODLER = Option::Some(func);
    }
}

pub fn _inner_register_key(hokey_str: String, callback_id: i32, add_to_list: bool) {
    let keyevent_info: KeyEventInfo = KeyEventInfo { key: hokey_str.clone(), callback_id: callback_id}; 
    if add_to_list {
        unsafe {
            CALLBACK_INFO_LIST.push(keyevent_info);
        }
    } 
    let hotkey = match hotkey::parse_hotkey(&hokey_str) {
        Ok(key) => key,
        Err(err) => return format_json_error(err),
    };
    // println!("_inner_register_key {}", callback_id);
    
    let hotkey_manager_clone = HOTKEY_MANAGER.clone();
    if let Err(err) = hotkey_manager_clone.lock().register(hotkey, move || {
        // println!("on callback {}", callback_id);
        // CALLBACK_FUNC(callback_id);
        unsafe {
            if !CALLBACK_HODLER.is_none() {
                let callback: extern fn(i32) = CALLBACK_HODLER.unwrap();
                callback(callback_id);
            }

            if !CALLBACK_RUST_HODLER.is_none() {
                let callback: fn(i32) = CALLBACK_RUST_HODLER.unwrap();
                callback(callback_id);
            }
        }
    }) {
        if let hotkey::HotkeyManagerError::HotkeyAlreadyRegistered(_) = err {
        } else {
            return format_json_error(err);
        }
    };
}   


pub fn _inner_unregister_key(hokey_str: String, callback_id: i32) {
    
    let hotkey = match hotkey::parse_hotkey(&hokey_str) {
        Ok(key) => key,
        Err(err) => return format_json_error(err),
    };
    // println!("_inner_unregister_key {}", callback_id);

    let hotkey_manager_clone = HOTKEY_MANAGER.clone();
    if let Err(err) = hotkey_manager_clone.lock().unregister(&hotkey) {
        return format_json_error(err);
    };
}

pub fn _inner_keydown_stop_keyevent() {
    // println!("_inner_keydown_stop_keyevent");
    unsafe {
        for (keyevent_info) in CALLBACK_INFO_LIST.iter() {
            _inner_unregister_key(keyevent_info.key.clone(), keyevent_info.callback_id);
        }
    }
}

pub fn _inner_keydown_resume_keyevent() {
    // println!("_inner_keydown_resume_keyevent");
    unsafe {
        for (keyevent_info) in CALLBACK_INFO_LIST.iter() {
            _inner_register_key(keyevent_info.key.clone(), keyevent_info.callback_id, false);
        }
    }
}