//! Enum wrappers for `CoreWLAN` integer types.

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
