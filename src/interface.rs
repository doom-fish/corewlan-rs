//! `CWInterface` wrapper.

use crate::{
    channel::Channel,
    configuration::Configuration,
    network::Network,
    object::{bool_result, collect_set, error_from_raw, null_err, optional_ptr, take_data_object, take_string_object, to_c_string_bytes, RetainedObject},
    types::{InterfaceMode, PhyMode, Security},
    Result,
};

#[derive(Debug, Clone)]
pub struct Interface {
    obj: RetainedObject,
}

impl Interface {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn interface_name(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_interface_name(self.as_raw())) }
    }

    #[must_use]
    pub fn power_on(&self) -> bool {
        unsafe { crate::ffi::cwrs_interface_power_on(self.as_raw()) }
    }

    #[must_use]
    pub fn supported_wlan_channels(&self) -> Vec<Channel> {
        unsafe {
            collect_set(crate::ffi::cwrs_interface_supported_wlan_channels(self.as_raw()))
                .into_iter()
                .filter_map(|raw| Channel::from_owned_raw(raw))
                .collect()
        }
    }

    #[must_use]
    pub fn wlan_channel(&self) -> Option<Channel> {
        unsafe { Channel::from_owned_raw(crate::ffi::cwrs_interface_wlan_channel(self.as_raw())) }
    }

    #[must_use]
    pub fn active_phy_mode(&self) -> PhyMode {
        unsafe { PhyMode::from_raw(crate::ffi::cwrs_interface_active_phy_mode(self.as_raw())) }
    }

    #[must_use]
    pub fn ssid(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_interface_ssid(self.as_raw())) }
    }

    #[must_use]
    pub fn ssid_data(&self) -> Option<Vec<u8>> {
        unsafe { take_data_object(crate::ffi::cwrs_interface_ssid_data(self.as_raw())) }
    }

    #[must_use]
    pub fn bssid(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_interface_bssid(self.as_raw())) }
    }

    #[must_use]
    pub fn rssi_value(&self) -> isize {
        unsafe { crate::ffi::cwrs_interface_rssi_value(self.as_raw()) }
    }

    #[must_use]
    pub fn noise_measurement(&self) -> isize {
        unsafe { crate::ffi::cwrs_interface_noise_measurement(self.as_raw()) }
    }

    #[must_use]
    pub fn security(&self) -> Security {
        unsafe { Security::from_raw(crate::ffi::cwrs_interface_security(self.as_raw())) }
    }

    #[must_use]
    pub fn transmit_rate(&self) -> f64 {
        unsafe { crate::ffi::cwrs_interface_transmit_rate(self.as_raw()) }
    }

    #[must_use]
    pub fn country_code(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_interface_country_code(self.as_raw())) }
    }

    #[must_use]
    pub fn interface_mode(&self) -> InterfaceMode {
        unsafe { InterfaceMode::from_raw(crate::ffi::cwrs_interface_mode(self.as_raw())) }
    }

    #[must_use]
    pub fn transmit_power(&self) -> isize {
        unsafe { crate::ffi::cwrs_interface_transmit_power(self.as_raw()) }
    }

    #[must_use]
    pub fn hardware_address(&self) -> Option<String> {
        unsafe { take_string_object(crate::ffi::cwrs_interface_hardware_address(self.as_raw())) }
    }

    #[must_use]
    pub fn service_active(&self) -> bool {
        unsafe { crate::ffi::cwrs_interface_service_active(self.as_raw()) }
    }

    #[must_use]
    pub fn cached_scan_results(&self) -> Vec<Network> {
        unsafe {
            collect_set(crate::ffi::cwrs_interface_cached_scan_results(self.as_raw()))
                .into_iter()
                .filter_map(|raw| Network::from_owned_raw(raw))
                .collect()
        }
    }

    #[must_use]
    pub fn configuration(&self) -> Option<Configuration> {
        unsafe {
            Configuration::from_owned_raw(crate::ffi::cwrs_interface_configuration(self.as_raw()))
        }
    }

    /// Perform a blocking Wi-Fi scan for the specified SSID bytes.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn scan_for_networks_with_ssid(&self, ssid: Option<&[u8]>) -> Result<Vec<Network>> {
        self.scan_for_networks_with_ssid_inner(ssid, None)
    }

    /// Perform a blocking Wi-Fi scan for the specified SSID bytes.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn scan_for_networks_with_ssid_include_hidden(
        &self,
        ssid: Option<&[u8]>,
        include_hidden: bool,
    ) -> Result<Vec<Network>> {
        self.scan_for_networks_with_ssid_inner(ssid, Some(include_hidden))
    }

    /// Perform a blocking Wi-Fi scan for the specified network name.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn scan_for_networks_with_name(&self, name: Option<&str>) -> Result<Vec<Network>> {
        self.scan_for_networks_with_name_inner(name, None)
    }

    /// Perform a blocking Wi-Fi scan for the specified network name.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn scan_for_networks_with_name_include_hidden(
        &self,
        name: Option<&str>,
        include_hidden: bool,
    ) -> Result<Vec<Network>> {
        self.scan_for_networks_with_name_inner(name, Some(include_hidden))
    }

    fn scan_for_networks_with_name_inner(
        &self,
        name: Option<&str>,
        include_hidden: Option<bool>,
    ) -> Result<Vec<Network>> {
        let name_bytes = name.map(to_c_string_bytes);
        let mut error = core::ptr::null_mut();
        let raw = unsafe {
            crate::ffi::cwrs_interface_scan_for_networks_with_name(
                self.as_raw(),
                name_bytes
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr().cast()),
                include_hidden.unwrap_or(false),
                include_hidden.is_some(),
                &mut error,
            )
        };

        if raw.is_null() {
            if error.is_null() {
                return null_err("-[CWInterface scanForNetworksWithName:error:]");
            }
            return Err(unsafe { error_from_raw("scanForNetworksWithName", error) });
        }

        Ok(unsafe {
            collect_set(raw)
                .into_iter()
                .filter_map(|raw| Network::from_owned_raw(raw))
                .collect()
        })
    }

    fn scan_for_networks_with_ssid_inner(
        &self,
        ssid: Option<&[u8]>,
        include_hidden: Option<bool>,
    ) -> Result<Vec<Network>> {
        let (ssid_ptr, ssid_len) = optional_ptr(ssid);
        let mut error = core::ptr::null_mut();
        let raw = unsafe {
            crate::ffi::cwrs_interface_scan_for_networks_with_ssid(
                self.as_raw(),
                ssid_ptr,
                ssid_len,
                include_hidden.unwrap_or(false),
                include_hidden.is_some(),
                &mut error,
            )
        };

        if raw.is_null() {
            if error.is_null() {
                return null_err("-[CWInterface scanForNetworksWithSSID:error:]");
            }
            return Err(unsafe { error_from_raw("scanForNetworksWithSSID", error) });
        }

        Ok(unsafe {
            collect_set(raw)
                .into_iter()
                .filter_map(|raw| Network::from_owned_raw(raw))
                .collect()
        })
    }

    /// Disable or enable the interface power state.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreWLAN`.
    pub fn set_power(&self, power_on: bool) -> Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::cwrs_interface_set_power(self.as_raw(), power_on, &mut error) };
        unsafe { bool_result(ok, error, "setPower:error:") }
    }
}
