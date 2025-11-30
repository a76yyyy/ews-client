# EWS Client Core - Integration Tests

This directory contains integration tests for the `ews-client-core` library. The test infrastructure provides mock servers, fixtures, and utilities for comprehensive testing.

## Quick Reference

### Basic Test Template

```rust
#[tokio::test]
async fn test_my_operation() {
    let mock = MockEwsServer::new().await;
    let response = create_folder_response("folder-123");
    mock.register_response(response).await;

    let client = create_test_client(&mock.ews_endpoint());
    // ... test code ...
    assert_response_success(&response);
}
```

### Common Patterns

**Generate IDs:**

```rust
let folder_id = generate_test_folder_id();
let item_id = generate_test_item_id();
let sync_state = generate_test_sync_state();
```

**Use Fixtures:**

```rust
create_folder_response(id)
delete_folder_response()
get_folder_response(id, name)
update_folder_response(id)
find_folder_response(id, name)
sync_folder_hierarchy_response(state, id)
```

**Make Assertions:**

```rust
assert_valid_soap_envelope(&response);
assert_response_success(&response);
assert_contains_folder_id(&response, "id");
assert_response_error(&response, "ErrorFolderNotFound");
```

**Mock Server Methods:**

```rust
mock.url()                              // Base URL
mock.ews_endpoint()                     // EWS endpoint URL
mock.register_response(response)        // Register 200 response
mock.register_response_with_status(code, response)  // Custom status
mock.mock_get_folder(id, name)          // Helper for GetFolder
mock.mock_create_item(id)               // Helper for CreateItem
mock.register_auth_error()              // 401 Unauthorized
mock.register_server_error()            // 500 Internal Server Error
mock.reset()                            // Clear all mocks
```

## Test Infrastructure

### 1. Mock EWS Server (`common/mock_server.rs`)

The `MockEwsServer` provides a simulated EWS endpoint for testing without requiring a real Exchange server.

**Features:**

- Starts on a random available port
- Supports custom SOAP responses
- Handles authentication errors
- Simulates server errors and timeouts
- Supports operation-specific mocking (e.g., `mock_get_folder`, `mock_create_item`)

**Usage:**

```rust
#[tokio::test]
async fn test_with_mock_server() {
    let mock = MockEwsServer::new().await;
    let endpoint = mock.ews_endpoint();

    // Register a response
    mock.register_response(create_folder_response("folder-123")).await;

    // Use the endpoint with your client
    let client = create_test_client(&endpoint);
    // ... test code ...
}
```

### 2. SOAP Response Fixtures (`common/fixtures.rs`)

Pre-built SOAP response templates based on Microsoft Learn documentation and the EwsServer.sys.mjs reference implementation.

**Available Fixtures:**

#### Folder Operations

- `create_folder_response(folder_id)`
- `delete_folder_response()`
- `get_folder_response(folder_id, display_name)`
- `get_folder_full_response(folder_id, parent_id, display_name, folder_class)`
- `get_folder_distinguished_response(distinguished_id, folder_id)`
- `update_folder_response(folder_id)`
- `copy_folder_response(folder_id)`
- `move_folder_response(folder_id)`
- `find_folder_response(folder_id, display_name)`
- `find_folder_paginated_response(folder_ids, display_names, total, includes_last)`
- `sync_folder_hierarchy_response(sync_state, folder_id)`
- `sync_folder_hierarchy_with_changes_response(sync_state, create_id, update_id, delete_id)`
- `sync_folder_hierarchy_paginated_response(old_state, new_state, folder_id)`

#### Item Operations

- `create_item_response(item_id)`
- `create_item_send_response()`
- `delete_item_response()`
- `get_item_response(item_id, subject)`
- `get_item_with_mime_response(item_id, subject, mime_content)`
- `get_item_full_response(...)`
- `get_item_with_recipients_response(...)`
- `get_item_with_extended_properties_response(...)`
- `get_item_with_body_response(...)`
- `update_item_response(item_id)`
- `update_item_set_field_response(item_id)`
- `update_item_append_field_response(item_id)`
- `update_item_delete_field_response(item_id)`
- `send_item_response()`
- `copy_item_response(item_id)`
- `move_item_response(item_id)`
- `find_item_response(item_id)`
- `find_item_paginated_response(item_ids, total, includes_last)`
- `sync_folder_items_response(sync_state, item_id)`
- `sync_folder_items_with_changes_response(sync_state, create_id, update_id, delete_id)`
- `sync_folder_items_paginated_response(old_state, new_state, item_id)`

#### Special Operations

- `mark_as_junk_response(item_id)`
- `mark_all_items_as_read_response()`

#### Batch Operations

- `batch_get_folder_response` / `batch_get_folder_mixed_response`
- `batch_get_item_response`
- `batch_delete_item_response` / `batch_delete_item_mixed_response`
- `batch_update_item_response` / `batch_update_item_mixed_response`
- `batch_mark_as_junk_response`
- `batch_copy_item_response` / `batch_move_item_response`
- `batch_copy_folder_response` / `batch_move_folder_response`
- `batch_delete_folder_mixed_response`

#### Error Responses

- `error_invalid_folder_id`
- `error_folder_not_found`
- `error_item_not_found`
- `error_access_denied`
- `error_server_busy`
- `error_invalid_request`
- `error_authentication_failed`
- `error_quota_exceeded`
- `error_insufficient_resources`
- `error_mailbox_store_unavailable`
- `error_folder_not_empty`
- `error_invalid_change_key`
- `error_invalid_sync_state`
- `error_mark_as_junk_not_supported`

### 3. Test Utilities (`common/test_utils.rs`)

Helper functions for creating clients and verifying responses.

**Client Creation:**

- `create_test_client(endpoint)`
- `create_test_client_oauth2(endpoint, token)`

**ID Generation:**

- `generate_test_folder_id()`
- `generate_test_item_id()`
- `generate_test_sync_state()`

**Assertions:**

- `assert_valid_soap_envelope(response)`
- `assert_response_success(response)`
- `assert_response_error(response, code)`
- `assert_response_fault(response, code, string)`
- `assert_contains_folder_id(response, id)`
- `assert_contains_item_id(response, id)`
- `assert_contains_display_name(response, name)`
- `assert_contains_subject(response, subject)`
- `assert_contains_sync_state(response, state)`

## Test Organization

Tests are organized by functionality:

```
tests/
├── lib.rs                          # Test library entry point
├── infrastructure_test.rs          # Framework validation tests
├── common/
│   ├── mod.rs                      # Common module exports
│   ├── mock_server.rs              # Mock EWS server
│   ├── fixtures.rs                 # SOAP response templates
│   └── test_utils.rs               # Helper functions
├── folder_operations.rs            # Folder operation integration tests (Live Server)
├── operations_test.rs              # Operation signature tests
└── README.md                       # This file
```

## Running Tests

### Run infrastructure tests (Mock based)

```bash
cargo test --test infrastructure_test
```

### Run integration tests (Requires Live Server)

These tests are marked with `#[ignore]` by default as they require a real EWS server.

1. Set environment variables:

   ```bash
   export EWS_ENDPOINT="https://outlook.office365.com/EWS/Exchange.asmx"
   export EWS_USERNAME="your_email@example.com"
   export EWS_PASSWORD="your_password"
   ```

2. Run ignored tests:

   ```bash
   cargo test --package ews-client-core -- --ignored
   ```

## Test Phases

### Phase 1: Infrastructure (Complete ✅)

- Mock EWS server
- SOAP response fixtures
- Test utilities
- Infrastructure validation tests

### Phase 2: Folder Operations (Complete ✅)

- Folder creation, deletion, update
- Folder retrieval (GetFolder, FindFolder)
- Folder hierarchy synchronization
- Copy/Move folders

### Phase 3: Item Operations (Complete ✅)

- Item creation, deletion, update
- Item retrieval (GetItem, FindItem)
- Item synchronization
- Copy/Move items
- Send message
- Mark as junk / Read status

### Phase 4: Error Handling (Pending)

- Authentication error tests
- Invalid ID error tests
- Server error tests
- Timeout handling tests
- Retry logic tests

### Phase 5: Advanced Features (Pending)

- Batch operations tests
- Concurrent request tests
- Performance tests
