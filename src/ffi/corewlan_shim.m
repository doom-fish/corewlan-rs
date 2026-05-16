#import <CoreWLAN/CoreWLAN.h>
#import <Foundation/Foundation.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static void *cwrs_retain_obj(id obj) {
    return obj ? (void *)CFBridgingRetain(obj) : NULL;
}

static void cwrs_set_error(void **error_out, NSError *error) {
    if (error_out != NULL) {
        *error_out = cwrs_retain_obj(error);
    }
}

static NSString *cwrs_string_from_utf8(const char *value) {
    return value ? [NSString stringWithUTF8String:value] : nil;
}

static NSData *cwrs_data_from_bytes(const uint8_t *bytes, size_t len) {
    return bytes ? [NSData dataWithBytes:bytes length:len] : nil;
}

static char *cwrs_strdup_nsstring(NSString *string) {
    if (string == nil) {
        return NULL;
    }

    const char *utf8 = [string UTF8String];
    if (utf8 == NULL) {
        return NULL;
    }

    size_t len = strlen(utf8);
    char *copy = malloc(len + 1);
    if (copy == NULL) {
        return NULL;
    }

    memcpy(copy, utf8, len + 1);
    return copy;
}

void *cwrs_retain(void *obj) {
    return cwrs_retain_obj((__bridge id)obj);
}

void cwrs_release(void *obj) {
    if (obj != NULL) {
        CFBridgingRelease(obj);
    }
}

void cwrs_free_buffer(void *buffer) {
    free(buffer);
}

char *cwrs_string_copy_utf8(void *string) {
    return cwrs_strdup_nsstring((__bridge NSString *)string);
}

uint8_t *cwrs_data_copy_bytes(void *data, size_t *len_out) {
    NSData *nsData = (__bridge NSData *)data;
    NSUInteger len = nsData.length;
    if (len_out != NULL) {
        *len_out = len;
    }

    if (len == 0) {
        return NULL;
    }

    uint8_t *copy = malloc(len);
    if (copy == NULL) {
        return NULL;
    }

    memcpy(copy, nsData.bytes, len);
    return copy;
}

size_t cwrs_array_count(void *array) {
    return ((__bridge NSArray *)array).count;
}

void *cwrs_array_object_at_index(void *array, size_t index) {
    NSArray *nsArray = (__bridge NSArray *)array;
    if (index >= nsArray.count) {
        return NULL;
    }
    return cwrs_retain_obj(nsArray[index]);
}

size_t cwrs_ordered_set_count(void *set) {
    return ((__bridge NSOrderedSet *)set).count;
}

void *cwrs_ordered_set_object_at_index(void *set, size_t index) {
    NSOrderedSet *orderedSet = (__bridge NSOrderedSet *)set;
    if (index >= orderedSet.count) {
        return NULL;
    }
    return cwrs_retain_obj([orderedSet objectAtIndex:index]);
}

void *cwrs_set_copy_all_objects(void *set) {
    NSSet *nsSet = (__bridge NSSet *)set;
    return cwrs_retain_obj(nsSet.allObjects);
}

NSInteger cwrs_error_code(void *error) {
    return ((__bridge NSError *)error).code;
}

char *cwrs_error_domain(void *error) {
    return cwrs_strdup_nsstring(((__bridge NSError *)error).domain);
}

char *cwrs_error_description(void *error) {
    return cwrs_strdup_nsstring(((__bridge NSError *)error).localizedDescription);
}

void *cwrs_wifi_client_shared(void) {
    return cwrs_retain_obj([CWWiFiClient sharedWiFiClient]);
}

void *cwrs_wifi_client_new(void) {
    return cwrs_retain_obj([[CWWiFiClient alloc] init]);
}

void *cwrs_wifi_client_interface(void *client) {
    return cwrs_retain_obj([(__bridge CWWiFiClient *)client interface]);
}

void *cwrs_wifi_client_interface_with_name(void *client, const char *name) {
    return cwrs_retain_obj([
        (__bridge CWWiFiClient *)client interfaceWithName:cwrs_string_from_utf8(name)
    ]);
}

void *cwrs_wifi_client_interfaces(void *client) {
    return cwrs_retain_obj([(__bridge CWWiFiClient *)client interfaces]);
}

void *cwrs_wifi_client_interface_names(void *client) {
    CWWiFiClient *wifiClient = (__bridge CWWiFiClient *)client;
    if ([wifiClient respondsToSelector:@selector(interfaceNames)]) {
        return cwrs_retain_obj([wifiClient interfaceNames]);
    }
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    return cwrs_retain_obj([CWWiFiClient interfaceNames]);
#pragma clang diagnostic pop
}

bool cwrs_wifi_client_start_monitoring_event(void *client, NSInteger event_type, void **error_out) {
    NSError *error = nil;
    BOOL ok = [(__bridge CWWiFiClient *)client startMonitoringEventWithType:(CWEventType)event_type error:&error];
    if (!ok) {
        cwrs_set_error(error_out, error);
    }
    return ok;
}

bool cwrs_wifi_client_stop_monitoring_event(void *client, NSInteger event_type, void **error_out) {
    NSError *error = nil;
    BOOL ok = [(__bridge CWWiFiClient *)client stopMonitoringEventWithType:(CWEventType)event_type error:&error];
    if (!ok) {
        cwrs_set_error(error_out, error);
    }
    return ok;
}

bool cwrs_wifi_client_stop_monitoring_all_events(void *client, void **error_out) {
    NSError *error = nil;
    BOOL ok = [(__bridge CWWiFiClient *)client stopMonitoringAllEventsAndReturnError:&error];
    if (!ok) {
        cwrs_set_error(error_out, error);
    }
    return ok;
}

void *cwrs_interface_name(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface interfaceName]);
}

bool cwrs_interface_power_on(void *interface) {
    return [(__bridge CWInterface *)interface powerOn];
}

void *cwrs_interface_supported_wlan_channels(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface supportedWLANChannels]);
}

void *cwrs_interface_wlan_channel(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface wlanChannel]);
}

NSInteger cwrs_interface_active_phy_mode(void *interface) {
    return [(__bridge CWInterface *)interface activePHYMode];
}

void *cwrs_interface_ssid(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface ssid]);
}

void *cwrs_interface_ssid_data(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface ssidData]);
}

void *cwrs_interface_bssid(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface bssid]);
}

NSInteger cwrs_interface_rssi_value(void *interface) {
    return [(__bridge CWInterface *)interface rssiValue];
}

NSInteger cwrs_interface_noise_measurement(void *interface) {
    return [(__bridge CWInterface *)interface noiseMeasurement];
}

NSInteger cwrs_interface_security(void *interface) {
    return [(__bridge CWInterface *)interface security];
}

double cwrs_interface_transmit_rate(void *interface) {
    return [(__bridge CWInterface *)interface transmitRate];
}

void *cwrs_interface_country_code(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface countryCode]);
}

NSInteger cwrs_interface_mode(void *interface) {
    return [(__bridge CWInterface *)interface interfaceMode];
}

NSInteger cwrs_interface_transmit_power(void *interface) {
    return [(__bridge CWInterface *)interface transmitPower];
}

void *cwrs_interface_hardware_address(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface hardwareAddress]);
}

bool cwrs_interface_service_active(void *interface) {
    return [(__bridge CWInterface *)interface serviceActive];
}

void *cwrs_interface_cached_scan_results(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface cachedScanResults]);
}

void *cwrs_interface_configuration(void *interface) {
    return cwrs_retain_obj([(__bridge CWInterface *)interface configuration]);
}

void *cwrs_interface_scan_for_networks_with_name(
    void *interface,
    const char *name,
    bool include_hidden,
    bool has_include_hidden,
    void **error_out
) {
    NSError *error = nil;
    CWInterface *wifiInterface = (__bridge CWInterface *)interface;
    NSSet<CWNetwork *> *result = has_include_hidden
        ? [wifiInterface scanForNetworksWithName:cwrs_string_from_utf8(name)
                                  includeHidden:include_hidden
                                          error:&error]
        : [wifiInterface scanForNetworksWithName:cwrs_string_from_utf8(name)
                                          error:&error];
    if (result == nil && error != nil) {
        cwrs_set_error(error_out, error);
    }
    return cwrs_retain_obj(result);
}

void *cwrs_interface_scan_for_networks_with_ssid(
    void *interface,
    const uint8_t *ssid_bytes,
    size_t ssid_len,
    bool include_hidden,
    bool has_include_hidden,
    void **error_out
) {
    NSError *error = nil;
    NSData *ssid = cwrs_data_from_bytes(ssid_bytes, ssid_len);
    CWInterface *wifiInterface = (__bridge CWInterface *)interface;
    NSSet<CWNetwork *> *result = has_include_hidden
        ? [wifiInterface scanForNetworksWithSSID:ssid
                                   includeHidden:include_hidden
                                           error:&error]
        : [wifiInterface scanForNetworksWithSSID:ssid error:&error];
    if (result == nil && error != nil) {
        cwrs_set_error(error_out, error);
    }
    return cwrs_retain_obj(result);
}

bool cwrs_interface_set_power(void *interface, bool power_on, void **error_out) {
    NSError *error = nil;
    BOOL ok = [(__bridge CWInterface *)interface setPower:power_on error:&error];
    if (!ok) {
        cwrs_set_error(error_out, error);
    }
    return ok;
}

void *cwrs_network_ssid(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network ssid]);
}

void *cwrs_network_ssid_data(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network ssidData]);
}

void *cwrs_network_bssid(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network bssid]);
}

void *cwrs_network_wlan_channel(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network wlanChannel]);
}

NSInteger cwrs_network_rssi_value(void *network) {
    return [(__bridge CWNetwork *)network rssiValue];
}

NSInteger cwrs_network_noise_measurement(void *network) {
    return [(__bridge CWNetwork *)network noiseMeasurement];
}

void *cwrs_network_information_element_data(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network informationElementData]);
}

void *cwrs_network_country_code(void *network) {
    return cwrs_retain_obj([(__bridge CWNetwork *)network countryCode]);
}

NSInteger cwrs_network_beacon_interval(void *network) {
    return [(__bridge CWNetwork *)network beaconInterval];
}

bool cwrs_network_ibss(void *network) {
    return [(__bridge CWNetwork *)network ibss];
}

bool cwrs_network_supports_security(void *network, NSInteger security) {
    return [(__bridge CWNetwork *)network supportsSecurity:(CWSecurity)security];
}

bool cwrs_network_supports_phy_mode(void *network, NSInteger phy_mode) {
    return [(__bridge CWNetwork *)network supportsPHYMode:(CWPHYMode)phy_mode];
}

NSInteger cwrs_channel_number(void *channel) {
    return [(__bridge CWChannel *)channel channelNumber];
}

NSInteger cwrs_channel_width(void *channel) {
    return [(__bridge CWChannel *)channel channelWidth];
}

NSInteger cwrs_channel_band(void *channel) {
    return [(__bridge CWChannel *)channel channelBand];
}

void *cwrs_configuration_network_profiles(void *configuration) {
    return cwrs_retain_obj([(__bridge CWConfiguration *)configuration networkProfiles]);
}

bool cwrs_configuration_require_admin_association(void *configuration) {
    return [(__bridge CWConfiguration *)configuration requireAdministratorForAssociation];
}

bool cwrs_configuration_require_admin_power(void *configuration) {
    return [(__bridge CWConfiguration *)configuration requireAdministratorForPower];
}

bool cwrs_configuration_require_admin_ibss_mode(void *configuration) {
    return [(__bridge CWConfiguration *)configuration requireAdministratorForIBSSMode];
}

bool cwrs_configuration_remember_joined_networks(void *configuration) {
    return [(__bridge CWConfiguration *)configuration rememberJoinedNetworks];
}

void *cwrs_network_profile_ssid(void *profile) {
    return cwrs_retain_obj([(__bridge CWNetworkProfile *)profile ssid]);
}

void *cwrs_network_profile_ssid_data(void *profile) {
    return cwrs_retain_obj([(__bridge CWNetworkProfile *)profile ssidData]);
}

NSInteger cwrs_network_profile_security(void *profile) {
    return [(__bridge CWNetworkProfile *)profile security];
}
