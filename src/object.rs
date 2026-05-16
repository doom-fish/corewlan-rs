//! Internal helpers for retained bridge objects and collection marshaling.

use crate::{error::CoreWlanError, ffi};
use core::{ffi::c_char, ptr};
use std::ffi::CStr;

#[derive(Debug)]
pub struct RetainedObject {
    raw: ffi::Object,
}

impl RetainedObject {
    pub unsafe fn from_owned_raw(raw: ffi::Object) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub const fn as_raw(&self) -> ffi::Object {
        self.raw
    }
}

impl Clone for RetainedObject {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::cwrs_retain(self.raw) };
        Self { raw }
    }
}

impl Drop for RetainedObject {
    fn drop(&mut self) {
        unsafe {
            ffi::cwrs_release(self.raw);
        }
    }
}

pub unsafe fn take_c_string(raw: *mut c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    let value = CStr::from_ptr(raw).to_string_lossy().into_owned();
    ffi::cwrs_free_buffer(raw.cast());
    Some(value)
}

pub unsafe fn take_data_buffer(raw: *mut u8, len: usize) -> Option<Vec<u8>> {
    if raw.is_null() {
        return (len == 0).then(Vec::new);
    }

    let slice = std::slice::from_raw_parts(raw, len);
    let value = slice.to_vec();
    ffi::cwrs_free_buffer(raw.cast());
    Some(value)
}

pub unsafe fn take_string_object(raw: ffi::Object) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    let value = take_c_string(ffi::cwrs_string_copy_utf8(raw));
    ffi::cwrs_release(raw);
    value
}

pub unsafe fn take_data_object(raw: ffi::Object) -> Option<Vec<u8>> {
    if raw.is_null() {
        return None;
    }

    let mut len = 0usize;
    let bytes = ffi::cwrs_data_copy_bytes(raw, &mut len);
    ffi::cwrs_release(raw);
    take_data_buffer(bytes, len)
}

pub unsafe fn collect_array(raw: ffi::Object) -> Vec<ffi::Object> {
    if raw.is_null() {
        return Vec::new();
    }

    let count = ffi::cwrs_array_count(raw);
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let object = ffi::cwrs_array_object_at_index(raw, index);
        if !object.is_null() {
            out.push(object);
        }
    }
    ffi::cwrs_release(raw);
    out
}

pub unsafe fn collect_ordered_set(raw: ffi::Object) -> Vec<ffi::Object> {
    if raw.is_null() {
        return Vec::new();
    }

    let count = ffi::cwrs_ordered_set_count(raw);
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let object = ffi::cwrs_ordered_set_object_at_index(raw, index);
        if !object.is_null() {
            out.push(object);
        }
    }
    ffi::cwrs_release(raw);
    out
}

pub unsafe fn collect_set(raw: ffi::Object) -> Vec<ffi::Object> {
    if raw.is_null() {
        return Vec::new();
    }

    let array = ffi::cwrs_set_copy_all_objects(raw);
    ffi::cwrs_release(raw);
    collect_array(array)
}

pub unsafe fn bool_result(
    ok: bool,
    error: ffi::Object,
    operation: &'static str,
) -> crate::Result<()> {
    if ok {
        if !error.is_null() {
            ffi::cwrs_release(error);
        }
        return Ok(());
    }

    if error.is_null() {
        return Err(CoreWlanError::OperationFailed(operation));
    }

    Err(error_from_raw(operation, error))
}

pub unsafe fn error_from_raw(operation: &'static str, error: ffi::Object) -> CoreWlanError {
    let code = ffi::cwrs_error_code(error);
    let domain = take_c_string(ffi::cwrs_error_domain(error))
        .unwrap_or_else(|| "NSCocoaErrorDomain".to_owned());
    let description = take_c_string(ffi::cwrs_error_description(error))
        .unwrap_or_else(|| operation.to_owned());
    ffi::cwrs_release(error);
    CoreWlanError::ObjectiveCError {
        operation,
        code,
        domain,
        description,
    }
}

pub const fn os_status_result(status: i32, operation: &'static str) -> crate::Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(CoreWlanError::OsStatusError { operation, status })
    }
}

pub const fn null_err<T>(what: &'static str) -> crate::Result<T> {
    Err(CoreWlanError::UnexpectedNull(what))
}

pub fn to_c_string_bytes(value: &str) -> Vec<u8> {
    let mut bytes = value
        .as_bytes()
        .iter()
        .copied()
        .filter(|byte| *byte != 0)
        .collect::<Vec<_>>();
    bytes.push(0);
    bytes
}

pub fn optional_ptr<T>(value: Option<&[T]>) -> (*const T, usize) {
    value.map_or((ptr::null(), 0), |slice| (slice.as_ptr(), slice.len()))
}
