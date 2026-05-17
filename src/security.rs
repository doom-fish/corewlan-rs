//! Type wrappers, utility helpers, and constants for `CoreWLAN` and related security objects.

use crate::{
    ffi,
    network::Network,
    object::{
        collect_array, collect_set, optional_ptr, os_status_result, take_string_object,
        to_c_string_bytes, RetainedObject,
    },
};
use bitflags::bitflags;
use core::ptr;

macro_rules! raw_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident = $value:expr,)*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            Unknown(isize),
        }

        impl $name {
            #[must_use]
            pub const fn from_raw(raw: isize) -> Self {
                match raw {
                    $($value => Self::$variant,)*
                    other => Self::Unknown(other),
                }
            }

            #[must_use]
            pub const fn as_raw(self) -> isize {
                match self {
                    $(Self::$variant => $value,)*
                    Self::Unknown(other) => other,
                }
            }
        }
    };
}

raw_enum! {
    /// Error codes corresponding to the `CWErrorDomain` domain.
    pub enum ErrorCode {
        NoErr = 0,
        EapolErr = 1,
        InvalidParameterErr = -3900,
        NoMemoryErr = -3901,
        UnknownErr = -3902,
        NotSupportedErr = -3903,
        InvalidFormatErr = -3904,
        TimeoutErr = -3905,
        UnspecifiedFailureErr = -3906,
        UnsupportedCapabilitiesErr = -3907,
        ReassociationDeniedErr = -3908,
        AssociationDeniedErr = -3909,
        AuthenticationAlgorithmUnsupportedErr = -3910,
        InvalidAuthenticationSequenceNumberErr = -3911,
        ChallengeFailureErr = -3912,
        ApFullErr = -3913,
        UnsupportedRateSetErr = -3914,
        ShortSlotUnsupportedErr = -3915,
        DsssOfdmUnsupportedErr = -3916,
        InvalidInformationElementErr = -3917,
        InvalidGroupCipherErr = -3918,
        InvalidPairwiseCipherErr = -3919,
        InvalidAkmPErr = -3920,
        UnsupportedRsnVersionErr = -3921,
        InvalidRsnCapabilitiesErr = -3922,
        CipherSuiteRejectedErr = -3923,
        InvalidPmkErr = -3924,
        SupplicantTimeoutErr = -3925,
        HtFeaturesNotSupportedErr = -3926,
        PcoTransitionTimeNotSupportedErr = -3927,
        ReferenceNotBoundErr = -3928,
        IpcFailureErr = -3929,
        OperationNotPermittedErr = -3930,
        Err = -3931,
    }
}

raw_enum! {
    /// IEEE 802.11 physical layer modes reported by `CoreWLAN`.
    pub enum PhyMode {
        None = 0,
        Ieee80211a = 1,
        Ieee80211b = 2,
        Ieee80211g = 3,
        Ieee80211n = 4,
        Ieee80211ac = 5,
        Ieee80211ax = 6,
        Ieee80211be = 7,
    }
}

raw_enum! {
    /// Operating mode for a Wi-Fi interface.
    pub enum InterfaceMode {
        None = 0,
        Station = 1,
        Ibss = 2,
        HostAp = 3,
    }
}

raw_enum! {
    /// Security types advertised by `CoreWLAN`.
    pub enum Security {
        None = 0,
        Wep = 1,
        WpaPersonal = 2,
        WpaPersonalMixed = 3,
        Wpa2Personal = 4,
        Personal = 5,
        DynamicWep = 6,
        WpaEnterprise = 7,
        WpaEnterpriseMixed = 8,
        Wpa2Enterprise = 9,
        Enterprise = 10,
        Wpa3Personal = 11,
        Wpa3Enterprise = 12,
        Wpa3Transition = 13,
        Owe = 14,
        OweTransition = 15,
    }
}

raw_enum! {
    /// IBSS security types used by the deprecated ad-hoc API.
    pub enum IbssModeSecurity {
        None = 0,
        Wep40 = 1,
        Wep104 = 2,
    }
}

raw_enum! {
    /// Width of a Wi-Fi channel.
    pub enum ChannelWidth {
        UnknownWidth = 0,
        Width20Mhz = 1,
        Width40Mhz = 2,
        Width80Mhz = 3,
        Width160Mhz = 4,
    }
}

raw_enum! {
    /// Frequency band of a Wi-Fi channel.
    pub enum ChannelBand {
        UnknownBand = 0,
        Ghz2 = 1,
        Ghz5 = 2,
        Ghz6 = 3,
    }
}

bitflags! {
    /// Cipher key flags used when setting WEP keys on an interface.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CipherKeyFlags: usize {
        const NONE = 0;
        const UNICAST = 1 << 1;
        const MULTICAST = 1 << 2;
        const TX = 1 << 3;
        const RX = 1 << 4;
    }
}

raw_enum! {
    /// Keychain domains used by the C utility helpers in `CoreWLAN`.
    pub enum KeychainDomain {
        None = 0,
        User = 1,
        System = 2,
    }
}

raw_enum! {
    /// `CoreWLAN` event types that may be monitored by `CWWiFiClient`.
    pub enum EventType {
        None = 0,
        PowerDidChange = 1,
        SsidDidChange = 2,
        BssidDidChange = 3,
        CountryCodeDidChange = 4,
        LinkDidChange = 5,
        LinkQualityDidChange = 6,
        ModeDidChange = 7,
        ScanCacheUpdated = 8,
        BtCoexStats = 9,
    }
}

#[derive(Debug, Clone)]
pub struct Identity {
    obj: RetainedObject,
}

impl Identity {
    /// # Safety
    ///
    /// `raw` must be an owned CoreFoundation `SecIdentityRef` retained by the bridge.
    pub unsafe fn from_owned_raw(raw: ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> ffi::Object {
        self.obj.as_raw()
    }
}

#[derive(Debug, Clone)]
pub struct Authorization {
    obj: RetainedObject,
}

impl Authorization {
    /// # Safety
    ///
    /// `raw` must be an owned `SFAuthorization` handle retained by the bridge.
    pub unsafe fn from_owned_raw(raw: ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> ffi::Object {
        self.obj.as_raw()
    }
}

#[must_use]
pub fn merge_networks(networks: &[Network]) -> Vec<Network> {
    let raw_networks = networks.iter().map(Network::as_raw).collect::<Vec<_>>();
    unsafe {
        collect_set(crate::ffi::cwrs_merge_networks(
            raw_networks.as_ptr(),
            raw_networks.len(),
        ))
        .into_iter()
        .filter_map(|raw| Network::from_owned_raw(raw))
        .collect()
    }
}

/// # Errors
///
/// Returns an `OSStatus` error if the password cannot be read from the selected keychain domain.
pub fn find_wifi_password(domain: KeychainDomain, ssid: &[u8]) -> crate::Result<Option<String>> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let mut password = ptr::null_mut();
    let status = unsafe {
        crate::ffi::cwrs_keychain_find_wifi_password(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            &mut password,
        )
    };
    os_status_result(status, "CWKeychainFindWiFiPassword")?;
    Ok(unsafe { take_string_object(password) })
}

/// # Errors
///
/// Returns an `OSStatus` error if the password cannot be stored in the selected keychain domain.
pub fn set_wifi_password(domain: KeychainDomain, ssid: &[u8], password: &str) -> crate::Result<()> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let password = to_c_string_bytes(password);
    let status = unsafe {
        crate::ffi::cwrs_keychain_set_wifi_password(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            password.as_ptr().cast(),
        )
    };
    os_status_result(status, "CWKeychainSetWiFiPassword")
}

/// # Errors
///
/// Returns an `OSStatus` error if the password cannot be deleted from the selected keychain domain.
pub fn delete_wifi_password(domain: KeychainDomain, ssid: &[u8]) -> crate::Result<()> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let status = unsafe {
        crate::ffi::cwrs_keychain_delete_wifi_password(domain.as_raw(), ssid_ptr, ssid_len)
    };
    os_status_result(status, "CWKeychainDeleteWiFiPassword")
}

/// # Errors
///
/// Returns an `OSStatus` error if the credentials cannot be read from the selected keychain domain.
pub fn find_wifi_eap_username_and_password(
    domain: KeychainDomain,
    ssid: &[u8],
) -> crate::Result<(Option<String>, Option<String>)> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let mut username = ptr::null_mut();
    let mut password = ptr::null_mut();
    let status = unsafe {
        crate::ffi::cwrs_keychain_find_wifi_eap_username_and_password(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            &mut username,
            &mut password,
        )
    };
    os_status_result(status, "CWKeychainFindWiFiEAPUsernameAndPassword")?;
    Ok((unsafe { take_string_object(username) }, unsafe {
        take_string_object(password)
    }))
}

/// # Errors
///
/// Returns an `OSStatus` error if the credentials cannot be stored in the selected keychain domain.
pub fn set_wifi_eap_username_and_password(
    domain: KeychainDomain,
    ssid: &[u8],
    username: Option<&str>,
    password: Option<&str>,
) -> crate::Result<()> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let username = username.map(to_c_string_bytes);
    let password = password.map(to_c_string_bytes);
    let status = unsafe {
        crate::ffi::cwrs_keychain_set_wifi_eap_username_and_password(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            username
                .as_ref()
                .map_or(ptr::null(), |value| value.as_ptr().cast()),
            password
                .as_ref()
                .map_or(ptr::null(), |value| value.as_ptr().cast()),
        )
    };
    os_status_result(status, "CWKeychainSetWiFiEAPUsernameAndPassword")
}

/// # Errors
///
/// Returns an `OSStatus` error if the credentials cannot be deleted from the selected keychain domain.
pub fn delete_wifi_eap_username_and_password(
    domain: KeychainDomain,
    ssid: &[u8],
) -> crate::Result<()> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let status = unsafe {
        crate::ffi::cwrs_keychain_delete_wifi_eap_username_and_password(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
        )
    };
    os_status_result(status, "CWKeychainDeleteWiFiEAPUsernameAndPassword")
}

/// # Errors
///
/// Returns an `OSStatus` error if the identity cannot be read from the selected keychain domain.
pub fn copy_wifi_eap_identity(
    domain: KeychainDomain,
    ssid: &[u8],
) -> crate::Result<Option<Identity>> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let mut identity = ptr::null_mut();
    let status = unsafe {
        crate::ffi::cwrs_keychain_copy_wifi_eap_identity(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            &mut identity,
        )
    };
    os_status_result(status, "CWKeychainCopyWiFiEAPIdentity")?;
    Ok(unsafe { Identity::from_owned_raw(identity) })
}

/// # Errors
///
/// Returns an `OSStatus` error if the identity cannot be stored in the selected keychain domain.
pub fn set_wifi_eap_identity(
    domain: KeychainDomain,
    ssid: &[u8],
    identity: Option<&Identity>,
) -> crate::Result<()> {
    let (ssid_ptr, ssid_len) = optional_ptr(Some(ssid));
    let status = unsafe {
        crate::ffi::cwrs_keychain_set_wifi_eap_identity(
            domain.as_raw(),
            ssid_ptr,
            ssid_len,
            identity.map_or(ptr::null_mut(), Identity::as_raw),
        )
    };
    os_status_result(status, "CWKeychainSetWiFiEAPIdentity")
}

/// # Errors
///
/// Returns an `OSStatus` error if the process cannot enumerate available EAP identities.
pub fn copy_eap_identity_list() -> crate::Result<Vec<Identity>> {
    let mut identities = ptr::null_mut();
    let status = unsafe { crate::ffi::cwrs_keychain_copy_eap_identity_list(&mut identities) };
    os_status_result(status, "CWKeychainCopyEAPIdentityList")?;
    Ok(unsafe {
        collect_array(identities)
            .into_iter()
            .filter_map(|raw| Identity::from_owned_raw(raw))
            .collect()
    })
}

fn constant_or_fallback(raw: ffi::Object, fallback: &'static str) -> String {
    unsafe { take_string_object(raw) }.unwrap_or_else(|| fallback.to_owned())
}

#[must_use]
pub fn error_domain() -> String {
    constant_or_fallback(unsafe { ffi::cwrs_cw_error_domain() }, "CWErrorDomain")
}

#[must_use]
pub fn power_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_power_did_change_notification() },
        "CWPowerDidChangeNotification",
    )
}

#[must_use]
pub fn ssid_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_ssid_did_change_notification() },
        "CWSSIDDidChangeNotification",
    )
}

#[must_use]
pub fn bssid_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_bssid_did_change_notification() },
        "CWBSSIDDidChangeNotification",
    )
}

#[must_use]
pub fn link_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_link_did_change_notification() },
        "CWLinkDidChangeNotification",
    )
}

#[must_use]
pub fn mode_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_mode_did_change_notification() },
        "CWModeDidChangeNotification",
    )
}

#[must_use]
pub fn country_code_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_country_code_did_change_notification() },
        "CWCountryCodeDidChangeNotification",
    )
}

#[must_use]
pub fn scan_cache_did_update_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_scan_cache_did_update_notification() },
        "CWScanCacheDidUpdateNotification",
    )
}

#[must_use]
pub fn link_quality_did_change_notification() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_link_quality_did_change_notification() },
        "CWLinkQualityDidChangeNotification",
    )
}

#[must_use]
pub fn link_quality_notification_rssi_key() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_link_quality_notification_rssi_key() },
        "CWLinkQualityNotificationRSSIKey",
    )
}

#[must_use]
pub fn link_quality_notification_transmit_rate_key() -> String {
    constant_or_fallback(
        unsafe { ffi::cwrs_link_quality_notification_transmit_rate_key() },
        "CWLinkQualityNotificationTransmitRateKey",
    )
}
