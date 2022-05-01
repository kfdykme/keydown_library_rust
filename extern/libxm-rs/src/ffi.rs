#![allow(non_camel_case_types, missing_copy_implementations)]

use libc::{c_char, c_float, c_int, size_t};

pub enum xm_context {}
pub type xm_context_t = xm_context;

extern "C" {
    pub fn xm_create_context_safe(
        context: *mut *mut xm_context_t,
        moddata: *const c_char,
        moddata_length: size_t,
        rate: u32,
    ) -> c_int;
    pub fn xm_free_context(context: *mut xm_context_t);
    pub fn xm_generate_samples(
        context: *mut xm_context_t,
        output: *mut c_float,
        numsamples: size_t,
    );
    pub fn xm_set_max_loop_count(context: *mut xm_context_t, loopcnt: u8);
    pub fn xm_get_loop_count(context: *mut xm_context_t) -> u8;
    pub fn xm_get_module_name(context: *mut xm_context_t) -> *const c_char;
    pub fn xm_get_tracker_name(context: *mut xm_context_t) -> *const c_char;
    pub fn xm_get_number_of_channels(context: *mut xm_context_t) -> u16;
    pub fn xm_get_module_length(context: *mut xm_context_t) -> u16;
    pub fn xm_get_number_of_patterns(context: *mut xm_context_t) -> u16;

    pub fn xm_get_number_of_rows(context: *mut xm_context_t, pattern: u16) -> u16;
    pub fn xm_get_number_of_instruments(context: *mut xm_context_t) -> u16;
    pub fn xm_get_number_of_samples(context: *mut xm_context_t, instrument: u16) -> u16;

    pub fn xm_get_playing_speed(context: *mut xm_context_t, bpm: *mut u16, tempo: *mut u16);
    pub fn xm_get_position(
        context: *mut xm_context_t,
        pattern_index: *mut u8,
        pattern: *mut u8,
        row: *mut u8,
        samples: *mut u64,
    );
    pub fn xm_get_latest_trigger_of_instrument(context: *mut xm_context_t, instrument: u16) -> u64;
    pub fn xm_get_latest_trigger_of_sample(
        context: *mut xm_context_t,
        instr: u16,
        sample: u16,
    ) -> u64;
    pub fn xm_get_latest_trigger_of_channel(context: *mut xm_context_t, channel: u16) -> u64;
}
