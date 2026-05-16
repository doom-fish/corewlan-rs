use super::Object;
use std::ffi::{c_char, c_void};

extern "C" {
    pub fn cwrs_retain(obj: Object) -> Object;
    pub fn cwrs_release(obj: Object);
    pub fn cwrs_free_buffer(buffer: *mut c_void);

    pub fn cwrs_string_copy_utf8(string: Object) -> *mut c_char;
    pub fn cwrs_data_copy_bytes(data: Object, len_out: *mut usize) -> *mut u8;

    pub fn cwrs_array_count(array: Object) -> usize;
    pub fn cwrs_array_object_at_index(array: Object, index: usize) -> Object;
    pub fn cwrs_ordered_set_count(set: Object) -> usize;
    pub fn cwrs_ordered_set_object_at_index(set: Object, index: usize) -> Object;
    pub fn cwrs_set_copy_all_objects(set: Object) -> Object;

    pub fn cwrs_error_code(error: Object) -> isize;
    pub fn cwrs_error_domain(error: Object) -> *mut c_char;
    pub fn cwrs_error_description(error: Object) -> *mut c_char;
}
