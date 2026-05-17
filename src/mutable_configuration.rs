//! `CWMutableConfiguration` wrapper.

use crate::{configuration::Configuration, error::CoreWlanError, network_profile::NetworkProfile};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct MutableConfiguration {
    base: Configuration,
}

impl MutableConfiguration {
    /// # Errors
    ///
    /// Returns an error if the framework unexpectedly returns `nil`.
    pub fn new() -> crate::Result<Self> {
        unsafe {
            Configuration::from_owned_raw(crate::ffi::cwrs_mutable_configuration_new())
                .map(|base| Self { base })
                .ok_or(CoreWlanError::UnexpectedNull(
                    "[[CWMutableConfiguration alloc] init]",
                ))
        }
    }

    /// # Errors
    ///
    /// Returns an error if creating the mutable copy unexpectedly yields `nil`.
    pub fn from_configuration(configuration: &Configuration) -> crate::Result<Self> {
        unsafe {
            Configuration::from_owned_raw(
                crate::ffi::cwrs_mutable_configuration_with_configuration(configuration.as_raw()),
            )
            .map(|base| Self { base })
            .ok_or(CoreWlanError::UnexpectedNull(
                "[CWConfiguration mutableCopy]",
            ))
        }
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.base.as_raw()
    }

    pub fn set_network_profiles(&self, profiles: &[NetworkProfile]) {
        let raw_profiles = profiles
            .iter()
            .map(NetworkProfile::as_raw)
            .collect::<Vec<_>>();
        unsafe {
            crate::ffi::cwrs_mutable_configuration_set_network_profiles(
                self.as_raw(),
                raw_profiles.as_ptr(),
                raw_profiles.len(),
            );
        }
    }

    pub fn set_require_administrator_for_association(&self, value: bool) {
        unsafe {
            crate::ffi::cwrs_mutable_configuration_set_require_admin_association(
                self.as_raw(),
                value,
            );
        }
    }

    pub fn set_require_administrator_for_power(&self, value: bool) {
        unsafe {
            crate::ffi::cwrs_mutable_configuration_set_require_admin_power(self.as_raw(), value);
        }
    }

    pub fn set_require_administrator_for_ibss_mode(&self, value: bool) {
        unsafe {
            crate::ffi::cwrs_mutable_configuration_set_require_admin_ibss_mode(
                self.as_raw(),
                value,
            );
        }
    }

    pub fn set_remember_joined_networks(&self, value: bool) {
        unsafe {
            crate::ffi::cwrs_mutable_configuration_set_remember_joined_networks(
                self.as_raw(),
                value,
            );
        }
    }
}

impl Default for MutableConfiguration {
    fn default() -> Self {
        Self::new().expect("CWMutableConfiguration init returned nil")
    }
}

impl Deref for MutableConfiguration {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl From<MutableConfiguration> for Configuration {
    fn from(value: MutableConfiguration) -> Self {
        value.base
    }
}

impl PartialEq for MutableConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Eq for MutableConfiguration {}
