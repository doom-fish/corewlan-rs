//! `CWNetwork` wrapper.

use crate::{
    channel::Channel,
    object::{take_data_object, take_string_object, RetainedObject},
    security::{PhyMode, Security},
};

#[derive(Debug, Clone)]
pub struct Network {
    obj: RetainedObject,
}

impl Network {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn ssid(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_network_ssid(self.as_raw())) }
    }

    #[must_use]
    pub fn ssid_data(&self) -> Option<Vec<u8>> {
        unsafe { take_data_object(crate::ffi::cwrs_network_ssid_data(self.as_raw())) }
    }

    #[must_use]
    pub fn bssid(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_network_bssid(self.as_raw())) }
    }

    #[must_use]
    pub fn wlan_channel(&self) -> Option<Channel> {
        unsafe { Channel::from_owned_raw(crate::ffi::cwrs_network_wlan_channel(self.as_raw())) }
    }

    #[must_use]
    pub fn rssi_value(&self) -> isize {
        unsafe { crate::ffi::cwrs_network_rssi_value(self.as_raw()) }
    }

    #[must_use]
    pub fn noise_measurement(&self) -> isize {
        unsafe { crate::ffi::cwrs_network_noise_measurement(self.as_raw()) }
    }

    #[must_use]
    pub fn information_element_data(&self) -> Option<Vec<u8>> {
        unsafe { take_data_object(crate::ffi::cwrs_network_information_element_data(self.as_raw())) }
    }

    #[must_use]
    pub fn country_code(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_network_country_code(self.as_raw())) }
    }

    #[must_use]
    pub fn beacon_interval(&self) -> isize {
        unsafe { crate::ffi::cwrs_network_beacon_interval(self.as_raw()) }
    }

    #[must_use]
    pub fn ibss(&self) -> bool {
        unsafe { crate::ffi::cwrs_network_ibss(self.as_raw()) }
    }

    #[must_use]
    pub fn supports_security(&self, security: Security) -> bool {
        unsafe { crate::ffi::cwrs_network_supports_security(self.as_raw(), security.as_raw()) }
    }

    #[must_use]
    pub fn supports_phy_mode(&self, mode: PhyMode) -> bool {
        unsafe { crate::ffi::cwrs_network_supports_phy_mode(self.as_raw(), mode.as_raw()) }
    }
}

impl PartialEq for Network {
    fn eq(&self, other: &Self) -> bool {
        unsafe { crate::ffi::cwrs_network_equal(self.as_raw(), other.as_raw()) }
    }
}

impl Eq for Network {}
