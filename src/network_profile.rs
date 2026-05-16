//! `CWNetworkProfile` wrapper.

use crate::{
    error::CoreWlanError,
    object::{take_data_object, take_string_object, RetainedObject},
    security::Security,
};

#[derive(Debug, Clone)]
pub struct NetworkProfile {
    obj: RetainedObject,
}

impl NetworkProfile {
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn new() -> crate::Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::cwrs_network_profile_new())
                .ok_or(CoreWlanError::UnexpectedNull("[[CWNetworkProfile alloc] init]"))
        }
    }

    /// # Errors
    ///
    /// Returns an error if copying the profile unexpectedly yields `nil`.
    pub fn from_network_profile(profile: &Self) -> crate::Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::cwrs_network_profile_with_network_profile(
                profile.as_raw(),
            ))
            .ok_or(CoreWlanError::UnexpectedNull("[CWNetworkProfile copy]"))
        }
    }

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

impl Default for NetworkProfile {
    fn default() -> Self {
        Self::new().expect("CWNetworkProfile init returned nil")
    }
}

impl PartialEq for NetworkProfile {
    fn eq(&self, other: &Self) -> bool {
        unsafe { crate::ffi::cwrs_network_profile_equal(self.as_raw(), other.as_raw()) }
    }
}

impl Eq for NetworkProfile {}
