use super::Object;

extern "C" {
    pub fn cwrs_channel_number(channel: Object) -> isize;
    pub fn cwrs_channel_width(channel: Object) -> isize;
    pub fn cwrs_channel_band(channel: Object) -> isize;
    pub fn cwrs_channel_equal(lhs: Object, rhs: Object) -> bool;
}
