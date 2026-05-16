//! `CWChannel` wrapper.

use crate::{
    object::RetainedObject,
    security::{ChannelBand, ChannelWidth},
};

#[derive(Debug, Clone)]
pub struct Channel {
    obj: RetainedObject,
}

impl Channel {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn channel_number(&self) -> isize {
        unsafe { crate::ffi::cwrs_channel_number(self.as_raw()) }
    }

    #[must_use]
    pub fn channel_width(&self) -> ChannelWidth {
        unsafe { ChannelWidth::from_raw(crate::ffi::cwrs_channel_width(self.as_raw())) }
    }

    #[must_use]
    pub fn channel_band(&self) -> ChannelBand {
        unsafe { ChannelBand::from_raw(crate::ffi::cwrs_channel_band(self.as_raw())) }
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        unsafe { crate::ffi::cwrs_channel_equal(self.as_raw(), other.as_raw()) }
    }
}

impl Eq for Channel {}
