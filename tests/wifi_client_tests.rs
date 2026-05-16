mod common;

use corewlan::{EventType, WiFiClient, WiFiClientEventDelegate};

struct TestDelegate;

impl WiFiClientEventDelegate for TestDelegate {}

#[test]
fn wifi_client_shared_and_names_smoke() {
    let _lock = common::corewlan_test_lock();
    let client = WiFiClient::shared().expect("shared client should exist");
    let names = client.interface_names();
    assert!(names.iter().all(|name| !name.is_empty()));
}

#[test]
fn wifi_client_delegate_round_trip() {
    let _lock = common::corewlan_test_lock();
    let client = WiFiClient::new().expect("client should exist");
    let registration = client.set_delegate(TestDelegate).expect("delegate install should succeed");
    assert!(client.has_delegate());
    drop(registration);
    assert!(!client.has_delegate());
}

#[test]
fn dropping_stale_registration_keeps_current_delegate_installed() {
    let _lock = common::corewlan_test_lock();
    let client = WiFiClient::new().expect("client should exist");
    let registration_one = client.set_delegate(TestDelegate).expect("first delegate install should succeed");
    let registration_two = client.set_delegate(TestDelegate).expect("second delegate install should succeed");

    assert!(client.has_delegate());
    drop(registration_one);
    assert!(client.has_delegate());
    drop(registration_two);
    assert!(!client.has_delegate());
}

#[test]
fn wifi_client_event_monitoring_smoke() {
    let _lock = common::corewlan_test_lock();
    let client = WiFiClient::new().expect("client should exist");
    client
        .start_monitoring_event(EventType::PowerDidChange)
        .expect("event registration should succeed");
    client
        .stop_monitoring_event(EventType::PowerDidChange)
        .expect("event unregistration should succeed");
}
