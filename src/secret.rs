use crate::{error::CoreWlanError, ffi};
use core::{
    ffi::c_char,
    fmt,
    sync::atomic::{compiler_fence, Ordering},
};
use std::ffi::CStr;

fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}

#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretString(<redacted>)")
    }
}

impl Drop for SecretString {
    fn drop(&mut self) {
        wipe(unsafe { self.0.as_bytes_mut() });
    }
}

pub struct SecretCString(Vec<u8>);

impl SecretCString {
    pub fn new(value: &str, argument: &'static str) -> crate::Result<Self> {
        if value.as_bytes().contains(&0) {
            return Err(CoreWlanError::InteriorNul(argument));
        }

        let mut bytes = Vec::with_capacity(value.len() + 1);
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(0);
        Ok(Self(bytes))
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr().cast()
    }
}

impl Drop for SecretCString {
    fn drop(&mut self) {
        wipe(&mut self.0);
    }
}

pub unsafe fn take_secret_string_object(raw: ffi::Object) -> Option<SecretString> {
    if raw.is_null() {
        return None;
    }

    let buffer = ffi::cwrs_string_copy_utf8(raw);
    ffi::cwrs_release(raw);
    if buffer.is_null() {
        return None;
    }

    let len = CStr::from_ptr(buffer).to_bytes().len();
    let bytes = std::slice::from_raw_parts_mut(buffer.cast::<u8>(), len);
    let value = SecretString(String::from_utf8_lossy(bytes).into_owned());
    wipe(bytes);
    ffi::cwrs_free_buffer(buffer.cast());
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::{wipe, SecretCString, SecretString};
    use crate::error::CoreWlanError;

    #[test]
    fn wipe_zeroes_every_byte() {
        let mut buffer = *b"hunter2";
        wipe(&mut buffer);
        assert_eq!(buffer, [0; 7]);
    }

    #[test]
    fn secret_string_debug_is_redacted() {
        let secret = SecretString("hunter2".to_owned());
        let cloned = secret.clone();
        let rendered = format!("{secret:?} {cloned:?}");
        assert!(!rendered.contains("hunter2"));
        assert_eq!(secret.expose(), "hunter2");
        assert_eq!(cloned.expose(), secret.expose());
    }

    #[test]
    fn secret_c_string_is_nul_terminated() {
        let secret = SecretCString::new("hunter2", "password").expect("no interior NUL");
        assert_eq!(secret.0, b"hunter2\0");
    }

    #[test]
    fn secret_c_string_rejects_interior_nul() {
        assert!(matches!(
            SecretCString::new("hun\0ter2", "password"),
            Err(CoreWlanError::InteriorNul("password"))
        ));
    }
}
