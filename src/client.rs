//! `CWWiFiClient` wrapper.

use crate::{
    error::CoreWlanError,
    interface::Interface,
    object::{bool_result, collect_array, take_string_object, to_c_string_bytes, RetainedObject},
    security::EventType,
    Result,
};
use core::{
    ffi::{c_char, c_void},
    ptr,
};
use std::{
    ffi::CStr,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};

#[allow(clippy::module_name_repetitions)]
pub trait WiFiClientEventDelegate: Send + Sync {
    fn client_connection_interrupted(&self) {}
    fn client_connection_invalidated(&self) {}
    fn power_state_did_change(&self, _interface_name: Option<String>) {}
    fn ssid_did_change(&self, _interface_name: Option<String>) {}
    fn bssid_did_change(&self, _interface_name: Option<String>) {}
    fn country_code_did_change(&self, _interface_name: Option<String>) {}
    fn link_did_change(&self, _interface_name: Option<String>) {}
    fn link_quality_did_change(
        &self,
        _interface_name: Option<String>,
        _rssi: isize,
        _transmit_rate: f64,
    ) {
    }
    fn mode_did_change(&self, _interface_name: Option<String>) {}
    fn scan_cache_updated(&self, _interface_name: Option<String>) {}
}

struct DelegateContext {
    delegate: Box<dyn WiFiClientEventDelegate>,
}

#[allow(clippy::module_name_repetitions)]
#[must_use = "keep the delegate registration alive for as long as callbacks should fire"]
pub struct DelegateRegistration {
    client: WiFiClient,
    context: Arc<DelegateContext>,
}

impl Drop for DelegateRegistration {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::cwrs_wifi_client_clear_delegate(
                self.client.as_raw(),
                Arc::as_ptr(&self.context).cast_mut().cast::<c_void>(),
            );
        }
    }
}

fn copy_optional_string(raw: *const c_char) -> Option<String> {
    if raw.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(raw) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn with_delegate(context: *mut c_void, callback: impl FnOnce(&dyn WiFiClientEventDelegate)) {
    if context.is_null() {
        return;
    }

    let context = unsafe { &*(context.cast::<DelegateContext>()) };
    let _ = catch_unwind(AssertUnwindSafe(|| callback(context.delegate.as_ref())));
}

extern "C" fn delegate_release_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }

    // This runs from Swift's delegate-bridge `deinit`, and dropping the
    // `Arc<DelegateContext>` invokes the user delegate's `Drop`. Guard against a
    // panic unwinding across the `extern "C"` boundary (UB), mirroring the
    // callback dispatch in `with_delegate`.
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        drop(Arc::from_raw(context.cast::<DelegateContext>()));
    }));
}

extern "C" fn client_connection_interrupted_callback(context: *mut c_void) {
    with_delegate(context, |delegate| delegate.client_connection_interrupted());
}

extern "C" fn client_connection_invalidated_callback(context: *mut c_void) {
    with_delegate(context, |delegate| delegate.client_connection_invalidated());
}

extern "C" fn power_state_did_change_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.power_state_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn ssid_did_change_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.ssid_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn bssid_did_change_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.bssid_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn country_code_did_change_callback(
    context: *mut c_void,
    interface_name: *const c_char,
) {
    with_delegate(context, |delegate| {
        delegate.country_code_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn link_did_change_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.link_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn link_quality_did_change_callback(
    context: *mut c_void,
    interface_name: *const c_char,
    rssi: isize,
    transmit_rate: f64,
) {
    with_delegate(context, |delegate| {
        delegate.link_quality_did_change(copy_optional_string(interface_name), rssi, transmit_rate);
    });
}

extern "C" fn mode_did_change_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.mode_did_change(copy_optional_string(interface_name));
    });
}

extern "C" fn scan_cache_updated_callback(context: *mut c_void, interface_name: *const c_char) {
    with_delegate(context, |delegate| {
        delegate.scan_cache_updated(copy_optional_string(interface_name));
    });
}

#[derive(Debug, Clone)]
pub struct WiFiClient {
    obj: RetainedObject,
}

impl WiFiClient {
    /// Return the process-wide shared `CoreWLAN` client.
    ///
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn shared() -> Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::cwrs_wifi_client_shared()).ok_or(
                crate::error::CoreWlanError::UnexpectedNull("+[CWWiFiClient sharedWiFiClient]"),
            )
        }
    }

    /// Create a new `CoreWLAN` client.
    ///
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn new() -> Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::cwrs_wifi_client_new()).ok_or(
                crate::error::CoreWlanError::UnexpectedNull("[[CWWiFiClient alloc] init]"),
            )
        }
    }

    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn interface(&self) -> Option<Interface> {
        unsafe { Interface::from_owned_raw(crate::ffi::cwrs_wifi_client_interface(self.as_raw())) }
    }

    #[must_use]
    pub fn interface_with_name(&self, name: Option<&str>) -> Option<Interface> {
        let name_bytes = name.map(to_c_string_bytes);
        unsafe {
            Interface::from_owned_raw(crate::ffi::cwrs_wifi_client_interface_with_name(
                self.as_raw(),
                name_bytes
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr().cast()),
            ))
        }
    }

    #[must_use]
    pub fn interfaces(&self) -> Vec<Interface> {
        unsafe {
            collect_array(crate::ffi::cwrs_wifi_client_interfaces(self.as_raw()))
                .into_iter()
                .filter_map(|raw| Interface::from_owned_raw(raw))
                .collect()
        }
    }

    #[must_use]
    pub fn interface_names(&self) -> Vec<String> {
        unsafe {
            collect_array(crate::ffi::cwrs_wifi_client_interface_names(self.as_raw()))
                .into_iter()
                .filter_map(|raw| take_string_object(raw))
                .collect()
        }
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { crate::ffi::cwrs_wifi_client_has_delegate(self.as_raw()) }
    }

    pub fn clear_delegate(&self) {
        unsafe {
            crate::ffi::cwrs_wifi_client_clear_delegate(self.as_raw(), ptr::null_mut());
        }
    }

    /// Register a Rust event delegate with the underlying `CWWiFiClient`.
    ///
    /// The returned [`DelegateRegistration`] must be kept alive for as long as
    /// callbacks should remain active.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge rejects the delegate registration.
    pub fn set_delegate<D>(&self, delegate: D) -> Result<DelegateRegistration>
    where
        D: WiFiClientEventDelegate + 'static,
    {
        let context = Arc::new(DelegateContext {
            delegate: Box::new(delegate),
        });
        let user_data = Arc::into_raw(Arc::clone(&context))
            .cast_mut()
            .cast::<c_void>();

        let ok = unsafe {
            crate::ffi::cwrs_wifi_client_set_delegate(
                self.as_raw(),
                Some(client_connection_interrupted_callback),
                Some(client_connection_invalidated_callback),
                Some(power_state_did_change_callback),
                Some(ssid_did_change_callback),
                Some(bssid_did_change_callback),
                Some(country_code_did_change_callback),
                Some(link_did_change_callback),
                Some(link_quality_did_change_callback),
                Some(mode_did_change_callback),
                Some(scan_cache_updated_callback),
                user_data,
                Some(delegate_release_context),
            )
        };

        if !ok {
            delegate_release_context(user_data);
            return Err(CoreWlanError::OperationFailed("setDelegate:"));
        }

        Ok(DelegateRegistration {
            client: self.clone(),
            context,
        })
    }

    /// Register for a `CoreWLAN` event type.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn start_monitoring_event(&self, event_type: EventType) -> Result<()> {
        let mut error = ptr::null_mut();
        let ok = unsafe {
            crate::ffi::cwrs_wifi_client_start_monitoring_event(
                self.as_raw(),
                event_type.as_raw(),
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "startMonitoringEventWithType:error:") }
    }

    /// Unregister a previously monitored `CoreWLAN` event type.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn stop_monitoring_event(&self, event_type: EventType) -> Result<()> {
        let mut error = ptr::null_mut();
        let ok = unsafe {
            crate::ffi::cwrs_wifi_client_stop_monitoring_event(
                self.as_raw(),
                event_type.as_raw(),
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "stopMonitoringEventWithType:error:") }
    }

    /// Unregister all monitored `CoreWLAN` events.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn stop_monitoring_all_events(&self) -> Result<()> {
        let mut error = ptr::null_mut();
        let ok = unsafe {
            crate::ffi::cwrs_wifi_client_stop_monitoring_all_events(self.as_raw(), &mut error)
        };
        unsafe { bool_result(ok, error, "stopMonitoringAllEventsAndReturnError:") }
    }
}

impl Default for WiFiClient {
    fn default() -> Self {
        Self::shared().expect("CWWiFiClient sharedWiFiClient returned nil")
    }
}
