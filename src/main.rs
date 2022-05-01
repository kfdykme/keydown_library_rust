use keydown::{_inner_register_key, _init_invoke_callback, _init_invoke_rust_callback};

use crate::keydown::_inner_keydown_stop_keyevent;  

mod keydown;
mod hotkey;
mod lib;
 

fn callback(callback_id: i32) {
    println!("callback {}", callback_id);
    _inner_keydown_stop_keyevent();
}

fn main() {
    
    _inner_register_key(String::from("ctrl+b"), 13, true);
    _init_invoke_rust_callback(callback);
    while true {
        
    }
} 