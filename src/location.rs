#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LocationAuthorization {
    ServicesDisabled,
    NotDetermined,
    Restricted,
    Denied,
    AuthorizedAlways,
    AuthorizedWhenInUse,
    Unknown(i32),
}

impl LocationAuthorization {
    const fn from_raw(raw: i32) -> Self {
        match raw {
            -1 => Self::ServicesDisabled,
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::AuthorizedAlways,
            4 => Self::AuthorizedWhenInUse,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::AuthorizedAlways | Self::AuthorizedWhenInUse)
    }
}

#[must_use]
pub fn location_authorization() -> LocationAuthorization {
    LocationAuthorization::from_raw(unsafe { crate::ffi::cwrs_location_authorization_status() })
}

#[cfg(test)]
mod tests {
    use super::LocationAuthorization;

    #[test]
    fn raw_statuses_map_to_core_location_values() {
        assert_eq!(
            LocationAuthorization::from_raw(-1),
            LocationAuthorization::ServicesDisabled
        );
        assert_eq!(
            LocationAuthorization::from_raw(0),
            LocationAuthorization::NotDetermined
        );
        assert_eq!(
            LocationAuthorization::from_raw(2),
            LocationAuthorization::Denied
        );
        assert_eq!(
            LocationAuthorization::from_raw(3),
            LocationAuthorization::AuthorizedAlways
        );
        assert_eq!(
            LocationAuthorization::from_raw(4),
            LocationAuthorization::AuthorizedWhenInUse
        );
        assert_eq!(
            LocationAuthorization::from_raw(42),
            LocationAuthorization::Unknown(42)
        );
    }

    #[test]
    fn only_granted_statuses_are_authorized() {
        assert!(LocationAuthorization::AuthorizedAlways.is_authorized());
        assert!(LocationAuthorization::AuthorizedWhenInUse.is_authorized());
        assert!(!LocationAuthorization::ServicesDisabled.is_authorized());
        assert!(!LocationAuthorization::NotDetermined.is_authorized());
        assert!(!LocationAuthorization::Restricted.is_authorized());
        assert!(!LocationAuthorization::Denied.is_authorized());
        assert!(!LocationAuthorization::Unknown(42).is_authorized());
    }
}
