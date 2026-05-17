//! `CWMutableNetworkProfile` wrapper.

use crate::{error::CoreWlanError, network_profile::NetworkProfile, security::Security};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct MutableNetworkProfile {
    base: NetworkProfile,
}

impl MutableNetworkProfile {
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn new() -> crate::Result<Self> {
        unsafe {
            NetworkProfile::from_owned_raw(crate::ffi::cwrs_mutable_network_profile_new())
                .map(|base| Self { base })
                .ok_or(CoreWlanError::UnexpectedNull(
                    "[[CWMutableNetworkProfile alloc] init]",
                ))
        }
    }

    /// # Errors
    ///
    /// Returns an error if creating the mutable copy unexpectedly yields `nil`.
    pub fn from_network_profile(profile: &NetworkProfile) -> crate::Result<Self> {
        unsafe {
            NetworkProfile::from_owned_raw(
                crate::ffi::cwrs_mutable_network_profile_with_network_profile(profile.as_raw()),
            )
            .map(|base| Self { base })
            .ok_or(CoreWlanError::UnexpectedNull(
                "[CWNetworkProfile mutableCopy]",
            ))
        }
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.base.as_raw()
    }

    pub fn set_ssid_data(&self, ssid: Option<&[u8]>) {
        let (ptr, len) = crate::object::optional_ptr(ssid);
        unsafe {
            crate::ffi::cwrs_mutable_network_profile_set_ssid_data(self.as_raw(), ptr, len);
        }
    }

    pub fn set_security(&self, security: Security) {
        unsafe {
            crate::ffi::cwrs_mutable_network_profile_set_security(self.as_raw(), security.as_raw());
        }
    }
}

impl Default for MutableNetworkProfile {
    fn default() -> Self {
        Self::new().expect("CWMutableNetworkProfile init returned nil")
    }
}

impl Deref for MutableNetworkProfile {
    type Target = NetworkProfile;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl From<MutableNetworkProfile> for NetworkProfile {
    fn from(value: MutableNetworkProfile) -> Self {
        value.base
    }
}

impl PartialEq for MutableNetworkProfile {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Eq for MutableNetworkProfile {}
