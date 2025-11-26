//! Server version detection and management.

use dashmap::DashMap;
use ews::server_version::ExchangeServerVersion;
use std::sync::LazyLock;
use url::Url;

/// The Exchange Server version to use in requests when we cannot figure out
/// which one to use (e.g. if the server hasn't provided us with a version
/// identifier yet). We default to Exchange Server 2007 SP1, which ensures
/// compatibility with older servers, while ensuring the server's behaviour
/// stays stable enough if we need to update that version number later on. SP1
/// specifically updates the format of EWS IDs to the one that is still used by
/// more modern servers, so it is preferable over plain Exchange Server 2007.
pub(super) const DEFAULT_EWS_SERVER_VERSION: ExchangeServerVersion = ExchangeServerVersion::Exchange2007_SP1;

/// Runtime cache for storing the mapping between EWS endpoints and their detected versions.
/// This cache persists for the lifetime of the application process.
///
/// Uses `DashMap` for lock-free concurrent access across multiple `EwsClient` instances.
static SERVER_VERSION_CACHE: LazyLock<DashMap<String, ExchangeServerVersion>> = LazyLock::new(DashMap::new);

/// Reads the version stored for a given EWS endpoint from the runtime cache.
///
/// If no version could be read for this endpoint, returns the default version.
pub(super) fn read_server_version(endpoint: &Url) -> ExchangeServerVersion {
    SERVER_VERSION_CACHE
        .get(endpoint.as_str())
        .map_or(DEFAULT_EWS_SERVER_VERSION, |entry| *entry.value())
}

/// Stores the server version for a given EWS endpoint in the runtime cache.
pub(super) fn store_server_version(endpoint: &Url, version: ExchangeServerVersion) {
    SERVER_VERSION_CACHE.insert(endpoint.to_string(), version);
}

/// Updates the server version from a `ServerVersionInfo` header.
///
/// This function:
/// 1. Parses the version string from the header
/// 2. Converts it to an `ExchangeServerVersion` enum
/// 3. Stores it in the runtime cache for future use
///
/// If the server provides an unknown version, defaults to `Exchange2013_SP1`.
pub(super) fn update_server_version_from_header(
    endpoint: &Url,
    header: ews::server_version::ServerVersionInfo,
) -> ExchangeServerVersion {
    let version = match header.version {
        Some(version) if !version.is_empty() => version,
        // If the server did not include a version identifier, return current cached version
        _ => return read_server_version(endpoint),
    };

    let version = ExchangeServerVersion::try_from(version.as_str()).unwrap_or_else(|_| {
        // If the server included an unknown version, default to the most recent known version
        log::warn!("Unknown server version '{version}' for endpoint {endpoint}, defaulting to Exchange2013_SP1");
        ExchangeServerVersion::Exchange2013_SP1
    });

    // Store the version in the runtime cache for future use
    store_server_version(endpoint, version);

    version
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_read_default_version() {
        let endpoint: Url = "https://test.example.com/EWS/Exchange.asmx".parse().unwrap();
        let version = read_server_version(&endpoint);
        assert_eq!(version, DEFAULT_EWS_SERVER_VERSION);
    }

    #[test]
    fn test_store_and_read_version() {
        let endpoint: Url = "https://test2.example.com/EWS/Exchange.asmx".parse().unwrap();
        let version = ExchangeServerVersion::Exchange2013_SP1;

        store_server_version(&endpoint, version);
        let read_version = read_server_version(&endpoint);

        assert_eq!(read_version, version);
    }

    #[test]
    fn test_update_from_header() {
        let endpoint: Url = "https://test3.example.com/EWS/Exchange.asmx".parse().unwrap();
        let header = ews::server_version::ServerVersionInfo {
            major_version: None,
            minor_version: None,
            major_build_number: None,
            minor_build_number: None,
            version: Some("Exchange2013_SP1".to_string()),
        };

        let version = update_server_version_from_header(&endpoint, header);
        assert_eq!(version, ExchangeServerVersion::Exchange2013_SP1);

        // Verify it was cached
        let cached_version = read_server_version(&endpoint);
        assert_eq!(cached_version, ExchangeServerVersion::Exchange2013_SP1);
    }

    #[test]
    fn test_update_from_header_unknown_version() {
        let endpoint: Url = "https://test4.example.com/EWS/Exchange.asmx".parse().unwrap();
        let header = ews::server_version::ServerVersionInfo {
            major_version: None,
            minor_version: None,
            major_build_number: None,
            minor_build_number: None,
            version: Some("Exchange2025_Unknown".to_string()),
        };

        let version = update_server_version_from_header(&endpoint, header);
        // Should default to Exchange2013_SP1 for unknown versions
        assert_eq!(version, ExchangeServerVersion::Exchange2013_SP1);
    }

    #[test]
    fn test_update_from_header_empty_version() {
        let endpoint: Url = "https://test5.example.com/EWS/Exchange.asmx".parse().unwrap();

        // Store a known version first
        store_server_version(&endpoint, ExchangeServerVersion::Exchange2010);

        let header = ews::server_version::ServerVersionInfo {
            major_version: None,
            minor_version: None,
            major_build_number: None,
            minor_build_number: None,
            version: None,
        };

        let version = update_server_version_from_header(&endpoint, header);
        // Should return cached version when header has no version
        assert_eq!(version, ExchangeServerVersion::Exchange2010);
    }
}
