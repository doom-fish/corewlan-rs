//! `CWConfiguration` wrapper.

use crate::{object::{collect_ordered_set, RetainedObject}, profile::NetworkProfile};

#[derive(Debug, Clone)]
pub struct Configuration {
    obj: RetainedObject,
}

impl Configuration {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn network_profiles(&self) -> Vec<NetworkProfile> {
        unsafe {
            collect_ordered_set(crate::ffi::cwrs_configuration_network_profiles(self.as_raw()))
                .into_iter()
                .filter_map(|raw| NetworkProfile::from_owned_raw(raw))
                .collect()
        }
    }

    #[must_use]
    pub fn require_administrator_for_association(&self) -> bool {
        unsafe { crate::ffi::cwrs_configuration_require_admin_association(self.as_raw()) }
    }

    #[must_use]
    pub fn require_administrator_for_power(&self) -> bool {
        unsafe { crate::ffi::cwrs_configuration_require_admin_power(self.as_raw()) }
    }

    #[must_use]
    pub fn require_administrator_for_ibss_mode(&self) -> bool {
        unsafe { crate::ffi::cwrs_configuration_require_admin_ibss_mode(self.as_raw()) }
    }

    #[must_use]
    pub fn remember_joined_networks(&self) -> bool {
        unsafe { crate::ffi::cwrs_configuration_remember_joined_networks(self.as_raw()) }
    }
}
