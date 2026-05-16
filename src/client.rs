//! `CWWiFiClient` wrapper.

use crate::{
    interface::Interface,
    object::{bool_result, collect_array, take_string_object, RetainedObject},
    types::EventType,
    Result,
};

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
            Self::from_owned_raw(crate::ffi::cwrs_wifi_client_shared())
                .ok_or(crate::error::CoreWlanError::UnexpectedNull(
                    "+[CWWiFiClient sharedWiFiClient]",
                ))
        }
    }

    /// Create a new `CoreWLAN` client.
    ///
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn new() -> Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::cwrs_wifi_client_new())
                .ok_or(crate::error::CoreWlanError::UnexpectedNull("[[CWWiFiClient alloc] init]"))
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
        let name_bytes = name.map(crate::object::to_c_string_bytes);
        unsafe {
            Interface::from_owned_raw(crate::ffi::cwrs_wifi_client_interface_with_name(
                self.as_raw(),
                name_bytes
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr().cast()),
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

    /// Register for a `CoreWLAN` event type.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn start_monitoring_event(&self, event_type: EventType) -> Result<()> {
        let mut error = core::ptr::null_mut();
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
        let mut error = core::ptr::null_mut();
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
        let mut error = core::ptr::null_mut();
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
