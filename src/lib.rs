
 
use std::ffi::CStr;
use std::str;
use libc::c_char;

use super::keydown::*;


#[no_mangle]
pub extern fn register_key(hotkey: * const i8, callback_id: i32) {
    // convert pointer into string
    let c_buf: *const c_char = hotkey;
    let c_str: &CStr = unsafe { CStr:: from_ptr(c_buf)};
    let str_slice: &str = c_str.to_str().unwrap();
    let str_hotkey_buf: String = str_slice.to_owned(); 

    _inner_register_key(str_hotkey_buf, callback_id, true);
}

#[no_mangle]
pub extern  fn keydown_stop_keyevent() {
    _inner_keydown_stop_keyevent();
}

#[no_mangle]
pub extern fn keydown_resume_keyenvent() {
    _inner_keydown_resume_keyevent();
}

#[no_mangle]
pub extern fn init_invoke_callback(func: extern fn(i32)) { 
    _init_invoke_callback(func); 
}