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

Tests are organized by type and functionality:

```
tests/
├── lib.rs                              # Test library entry point
├── README.md                           # This file
│
├── common/                             # 🔧 Common test infrastructure
│   ├── mod.rs                          # Module exports
│   ├── mock_server.rs                  # Mock EWS server implementation
│   ├── fixtures.rs                     # SOAP response templates
│   └── test_utils.rs                   # Helper functions and assertions
│
├── integration/                        # 🧪 Integration tests
│   ├── mock/                           # Mock server-based tests (no real server needed)
│   │   ├── infrastructure.rs           # Framework validation
│   │   ├── folder_operations.rs        # Folder operations
│   │   └── item_operations.rs          # Item operations
│   └── real/                           # Real EWS server tests (marked #[ignore])
│       ├── folder_operations.rs        # Folder operations with live server
│       └── item_operations.rs          # Item operations with live server
│
├── unit/                               # 🔬 Unit tests
│   └── operations.rs                   # Operation signature tests
│
├── integration_mock.rs                 # Entry point for mock integration tests
├── integration_real.rs                 # Entry point for real server tests
└── unit.rs                             # Entry point for unit tests
```

## Running Tests

### Run all tests (Recommended)

```bash
# Run all tests (mock + unit, excludes real server tests)
cargo test

# Run specific test suites
cargo test --test integration_mock    # Mock integration tests (118 tests)
cargo test --test unit                # Unit tests (5 tests)
cargo test --test integration_real    # Real server tests (16 tests, all ignored)
```

### Run mock-based integration tests (No server required) ✨ RECOMMENDED

These tests use the mock server and can run in CI/CD without any external dependencies:

```bash
cargo test --test integration_mock
```

**What these tests do:**

- ✅ Start a real HTTP server on a random port
- ✅ Send actual HTTP POST requests with SOAP XML
- ✅ Verify the mock server returns correct SOAP responses
- ✅ Test success cases, error cases, batch operations, and pagination
- ✅ **No real EWS server needed!**
- ✅ **Fast execution: ~0.06s for 118 tests**

**Test coverage:**

- Infrastructure validation (8 tests)
- Folder operations (16 tests)
- Item operations (21 tests)
- Common module tests (73 tests)

### Run unit tests

```bash
cargo test --test unit
```

These tests verify operation signatures and basic functionality without network calls.

### Run real server integration tests (Requires Live Server)

These tests are marked with `#[ignore]` by default as they require a real EWS server.

1. Set environment variables:

   ```bash
   export EWS_ENDPOINT="https://outlook.office365.com/EWS/Exchange.asmx"
   export EWS_USERNAME="your_email@example.com"
   export EWS_PASSWORD="your_password"
   ```

2. Run ignored tests:

   ```bash
   cargo test --test integration_real -- --ignored
   # Or run all ignored tests in the package
   cargo test --package ews-client-core -- --ignored
   ```

## Test Phases

### Phase 1: Infrastructure (Complete ✅)

- Mock EWS server
- SOAP response fixtures
- Test utilities
- Infrastructure validation tests

### Phase 2: Folder Operations (Complete ✅)

**Mock Tests (mock_folder_operations_test.rs):**

- ✅ Create, delete, update folders
- ✅ Get folder (single and batch)
- ✅ Find folders (with pagination)
- ✅ Copy/Move folders
- ✅ Sync folder hierarchy (with changes)
- ✅ Error handling (folder not found, access denied)
- ✅ Authentication errors (401)
- ✅ Server errors (500)
- ✅ Batch operations with mixed results

**Live Server Tests (folder_operations.rs):**

- ✅ Real EWS server integration tests (marked with `#[ignore]`)

### Phase 3: Item Operations (Complete ✅)

**Mock Tests (mock_item_operations_test.rs):**

- ✅ Create, delete, update items
- ✅ Get item (single and batch, with MIME content)
- ✅ Find items (with pagination)
- ✅ Copy/Move items
- ✅ Send items
- ✅ Sync folder items (with changes)
- ✅ Mark as junk / Mark all as read
- ✅ Error handling (item not found, invalid change key)
- ✅ Batch operations (get, delete, update, copy, move, mark as junk)
- ✅ Batch operations with mixed results

**Live Server Tests (item_operations.rs):**

- ✅ Real EWS server integration tests (marked with `#[ignore]`)

### Phase 4: Error Handling (Complete ✅)

- ✅ Authentication error tests (401 Unauthorized)
- ✅ Invalid ID error tests (ErrorFolderNotFound, ErrorItemNotFound)
- ✅ Server error tests (500 Internal Server Error)
- ✅ Access denied tests (ErrorAccessDenied)
- ✅ Invalid change key tests (ErrorInvalidChangeKey)
- ✅ Batch operations with partial failures

### Phase 5: Advanced Features (Complete ✅)

- ✅ Batch operations tests (folders and items)
- ✅ Pagination tests (FindFolder, FindItem)
- ✅ Sync operations with change tracking
- ✅ Mixed success/error responses

## Test Coverage Summary

| Test Suite | Location | Tests | Status |
|------------|----------|-------|--------|
| **Mock Integration** | `integration_mock.rs` | **118** | ✅ All passing |
| - Infrastructure | `integration/mock/infrastructure.rs` | 8 | ✅ |
| - Folder Operations | `integration/mock/folder_operations.rs` | 16 | ✅ |
| - Item Operations | `integration/mock/item_operations.rs` | 21 | ✅ |
| - Common Tests | `common/*` | 73 | ✅ |
| **Unit Tests** | `unit.rs` | **5** | ✅ All passing |
| - Operation Signatures | `unit/operations.rs` | 5 | ✅ |
| **Real Integration** | `integration_real.rs` | **16** | ⚠️ Requires server |
| - Folder Operations | `integration/real/folder_operations.rs` | 7 | ⚠️ |
| - Item Operations | `integration/real/item_operations.rs` | 9 | ⚠️ |
| **Total** | | **139** | **123 passing without server** |

### Test Execution Time

- Mock integration tests: ~0.06s
- Unit tests: ~0.04s
- **Total (no server needed): ~0.10s** ⚡️
