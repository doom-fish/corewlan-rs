mod common;

#[test]
fn channel_round_trip_and_equality() {
    if let Some(channel) = common::first_channel() {
        let clone = channel.clone();
        assert_eq!(channel, clone);
        let _ = channel.channel_number();
        let _ = channel.channel_width();
        let _ = channel.channel_band();
    }
}
