//! `CWNetworkProfile` wrapper.

use crate::{object::{RetainedObject, take_data_object, take_string_object}, types::Security};

#[derive(Debug, Clone)]
pub struct NetworkProfile {
    obj: RetainedObject,
}

impl NetworkProfile {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn ssid(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_network_profile_ssid(self.as_raw())) }
    }

    #[must_use]
    pub fn ssid_data(&self) -> Option<Vec<u8>> {
        unsafe { take_data_object(crate::ffi::cwrs_network_profile_ssid_data(self.as_raw())) }
    }

    #[must_use]
    pub fn security(&self) -> Security {
        unsafe { Security::from_raw(crate::ffi::cwrs_network_profile_security(self.as_raw())) }
    }
}
