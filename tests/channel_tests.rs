mod common;

#[test]
fn channel_round_trip_and_equality() {
    if let Some(channel) = common::first_channel() {
        let clone = channel.clone();
        assert_eq!(channel, clone);
        assert!(channel.channel_number() > 0);
        assert_eq!(clone.channel_number(), channel.channel_number());
        assert_eq!(clone.channel_width(), channel.channel_width());
        assert_eq!(clone.channel_band(), channel.channel_band());
    }
}
