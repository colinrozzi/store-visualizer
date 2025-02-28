# Theater Store Visualizer Specification

## 1. Overview

The Store Visualizer is a WebAssembly actor that provides a visual interface for exploring and managing the Theater runtime's content store. It connects to the Theater runtime's store interface, displaying all stored entries, labels, and content in a user-friendly web interface.

## 2. Key Features

1. **Content Store Exploration**
   - View all content references stored in the Theater runtime
   - Filter content by hash, label, or content type
   - View detailed information about each content entry

2. **Label Management**
   - List all labels in the store
   - View all content associated with a specific label
   - Create, edit, and delete labels
   - Associate/disassociate content with labels

3. **Content Management**
   - View raw content and formatted representations (JSON, text, etc.)
   - Add new content to the store
   - Replace content at specific labels
   - Delete content from the store

4. **Visualization Features**
   - Content size visualization
   - Label relationship visualization
   - Storage usage statistics

## 3. Technical Architecture

### 3.1. Interface Alignment

The store visualizer will directly use the Theater store interface defined in `store.wit`, which provides the following operations:

```wit
interface store {
    /// Reference to content in the store
    record content-ref {
        hash: string,
    }

    /// Store content and return its ContentRef
    store: func(content: list<u8>) -> result<content-ref, string>;
    
    /// Retrieve content by its reference
    get: func(content-ref: content-ref) -> result<list<u8>, string>;
    
    /// Check if content exists
    exists: func(content-ref: content-ref) -> result<bool, string>;
    
    /// Attach a label to content
    label: func(label: string, content-ref: content-ref) -> result<_, string>;
    
    /// Get content references by label
    get-by-label: func(label: string) -> result<list<content-ref>, string>;
    
    /// Remove a label
    remove-label: func(label: string) -> result<_, string>;
    
    /// Remove a specific content reference from a label
    remove-from-label: func(label: string, content-ref: content-ref) -> result<_, string>;
    
    /// Store content and immediately label it
    put-at-label: func(label: string, content: list<u8>) -> result<content-ref, string>;
    
    /// Put content at a label, replacing any existing content
    replace-content-at-label: func(label: string, content: list<u8>) -> result<content-ref, string>;
    
    /// Replace content at a label with a specific content reference
    replace-at-label: func(label: string, content-ref: content-ref) -> result<_, string>;
    
    /// List all labels
    list-labels: func() -> result<list<string>, string>;
    
    /// List all content references
    list-all-content: func() -> result<list<content-ref>, string>;
    
    /// Calculate total size of all content
    calculate-total-size: func() -> result<u64, string>;
}
```

### 3.2. Actor Structure

The store-visualizer will be implemented as a Theater WebAssembly actor with:

1. **HTTP Server Component**
   - Serves the web UI (HTML, CSS, JavaScript)
   - Provides REST API endpoints for store operations

2. **Interface Implementation**
   - Implements the Theater actor interface
   - Connects to the runtime for store access
   - Handles initialization with store configuration

### 3.3. Web API Structure

The visualizer's HTTP API will directly map to these store operations:

| Store WIT Function | HTTP Endpoint | Method | Description |
|--------------------|---------------|--------|-------------|
| `store` | `/api/content` | POST | Upload new content to store |
| `get` | `/api/content/:hash` | GET | Retrieve content by hash |
| `exists` | `/api/content/:hash/exists` | GET | Check if content exists |
| `list-all-content` | `/api/content` | GET | List all content (paginated) |
| `calculate-total-size` | `/api/stats/size` | GET | Get total store size |
| `label` | `/api/labels/:name/content/:hash` | PUT | Attach label to content |
| `get-by-label` | `/api/labels/:name` | GET | Get content for label |
| `remove-label` | `/api/labels/:name` | DELETE | Remove a label |
| `remove-from-label` | `/api/labels/:name/content/:hash` | DELETE | Remove content from label |
| `put-at-label` | `/api/labels/:name/content` | POST | Store content with label |
| `replace-content-at-label` | `/api/labels/:name/content` | PUT | Replace content at label |
| `replace-at-label` | `/api/labels/:name/content/:hash` | PUT | Replace with existing content |
| `list-labels` | `/api/labels` | GET | List all labels |

## 4. User Interface

### 4.1. Main Components

The web UI will consist of:

1. **Main Dashboard**
   - Overview of store statistics (total items, size, label count)
   - Quick access to recently accessed content
   - Search and filter controls

2. **Content Explorer**
   - Paginated list of all content with hash, size, and type
   - Filtering by hash prefix, content type, size range
   - Sorting options (by hash, size, date added)

3. **Label Explorer**
   - List of all labels with associated content count
   - Ability to create new labels and manage existing ones
   - Visual indicator of label relationships

4. **Content Detail View**
   - Raw and formatted content display
   - Label associations and management
   - Content metadata and operations (copy, download, etc.)

### 4.2. UI Layout

The UI will have a responsive layout with:

- Top navigation bar for switching between views
- Left sidebar for filters and search
- Main content area for data display
- Right panel for details and actions
- Bottom status bar for system information

### 4.3. UI Mockups

**Dashboard View**
```
+--------------------------------------------+
| Theater Store Visualizer             [🔍] |
+-------------+----------------------------+-+
| NAVIGATION  | STORE STATISTICS          |D|
| Dashboard   | Total Items: 245          |E|
| Content     | Total Size: 1.2 MB        |T|
| Labels      | Labels: 37                |A|
|             |                           |I|
|-------------| RECENT CONTENT            |L|
| FILTERS     | - config.json (5 min ago) | |
| Hash        | - user.data (1 hour ago)  |P|
| Size        | - log.txt (2 hours ago)   |A|
| Type        |                           |N|
|             | LABEL DISTRIBUTION        |E|
|             | [Chart visualization]     |L|
+-------------+----------------------------+-+
```

**Content Explorer View**
```
+--------------------------------------------+
| Theater Store Visualizer             [🔍] |
+-------------+----------------------------+-+
| NAVIGATION  | CONTENT (245 items)        |D|
| Dashboard   | [Refresh] [Upload]         |E|
| Content     |                           |T|
| Labels      | Hash          Size  Type   |A|
|             |----------------------------|I|
|-------------| a1b2c3d4...  12KB  JSON    |L|
| FILTERS     | e5f6g7h8...  542B  Text    | |
| Hash        | i9j0k1l2...  2.3MB Binary  |P|
| Size        | m3n4o5p6...  89KB  JSON    |A|
| Type        | ...                        |N|
|             |                           |E|
|             | < 1 2 3 ... 25 >          |L|
+-------------+----------------------------+-+
```

**Content Detail View**
```
+--------------------------------------------+
| Theater Store Visualizer             [🔍] |
+-------------+----------------------------+-+
| NAVIGATION  | CONTENT DETAILS            |A|
| Dashboard   | Hash: a1b2c3d4...          |C|
| Content     | Size: 12KB                 |T|
| Labels      | Type: JSON                 |I|
|             |                           |O|
|-------------| CONTENT PREVIEW           |N|
| ASSOCIATED  | {                          |S|
| LABELS      |   "name": "Example",       | |
| - config    |   "value": 42,             |C|
| - user      |   "items": [               |O|
| - settings  |     "one",                 |P|
|             |     "two"                 |Y|
|             |   ]                        | |
|             | }                          |D|
|             |                           |L|
+-------------+----------------------------+-+
```

## 5. Data Model

### 5.1. Core Types

```rust
// Content reference (matches WIT interface)
struct ContentRef {
    hash: String,
}

// Extended content item for UI
struct ContentItem {
    hash: String,
    size: usize,
    preview: String,        // Content preview/summary
    content_type: String,   // Detected content type
    labels: Vec<String>,    // Associated labels
}

// Label with associated content
struct LabelItem {
    name: String,
    content_refs: Vec<ContentRef>,
}

// Store statistics
struct StoreStats {
    total_content_count: usize,
    total_size_bytes: u64,
    label_count: usize,
}
```

### 5.2. Content Type Detection

The visualizer will attempt to detect content types based on content analysis:

1. **JSON**: Valid JSON structure
2. **Text**: Valid UTF-8 text
3. **HTML**: Content starting with HTML tags
4. **Binary**: Any other content

## 6. Implementation Requirements

### 6.1 Technology Stack

1. **Backend**
   - Rust with WebAssembly compilation target
   - Theater actor model and interfaces
   - JSON serialization for API communication

2. **Frontend**
   - HTML, CSS, and JavaScript (vanilla or minimal framework)
   - Fetch API for AJAX communication
   - Simple charting library for visualizations

### 6.2 Development Phases

1. **Phase 1: Core Functionality**
   - Basic actor setup with HTTP server
   - Content listing and viewing
   - Simple label management

2. **Phase 2: Enhanced UI**
   - Improved content browsing with pagination
   - Advanced content type detection and formatting
   - Content search and filtering

3. **Phase 3: Advanced Features**
   - Visualizations and statistics
   - Content upload and editing
   - Enhanced label management

### 6.3 Actor Manifest

```toml
name = "store-visualizer"
version = "0.1.0"
description = "Visualization interface for Theater content store"
component_path = "target/wasm32-unknown-unknown/release/store_visualizer.wasm"
init_data = "assets/init.json"

[interface]
implements = "ntwk:theater/actor"
requires = ["ntwk:theater/store"]

[[handlers]]
type = "http-server"
config = { port = 8090 }

[[handlers]]
type = "filesystem"
config = { path = "assets" }
```

### 6.4 Initialization Data

```json
{
  "store_path": "store",
  "page_size": 20,
  "cache_enabled": true
}
```

## 7. User Workflows

### 7.1. Content Browsing

1. User opens the visualizer web interface
2. Dashboard shows statistics and recent content
3. User navigates to Content Explorer
4. User can browse all content with pagination
5. User can filter content by criteria
6. User selects content to view details
7. Content is displayed in appropriate format

### 7.2. Label Management

1. User navigates to Labels section
2. User views existing labels and associated content
3. User creates new label or selects existing label
4. User can associate/disassociate content with label
5. User can delete labels if needed

### 7.3. Content Management

1. User uploads new content
2. System processes and stores content
3. User assigns labels to content
4. User can view, download, or manage content

## 8. Security and Performance

### 8.1. Security Considerations

1. Input validation for all user-provided data
2. Content size limits for uploads
3. Content type validation
4. Error handling without information leakage

### 8.2. Performance Optimization

1. Pagination for large content lists
2. Content preview generation for large files
3. Caching of frequently accessed content
4. Asynchronous loading for UI responsiveness

## 9. Future Extensions

1. **Advanced Search**
   - Full-text search within content
   - Regular expression search
   - Advanced filtering options

2. **Content Versioning**
   - View content history
   - Restore previous versions

3. **Export/Import**
   - Export selected content or labels
   - Import content with custom labels

4. **Integration Features**
   - Integration with other Theater actors
   - Visualization of content relationships

## 10. Resources and Dependencies

1. **Required Libraries**
   - `serde` and `serde_json` for serialization
   - `base64` for encoding binary content
   - Simple charting library for frontend visualizations

2. **Asset Requirements**
   - HTML, CSS, and JavaScript files
   - Icons and UI elements
   - Documentation files

## 11. Testing Strategy

1. **Unit Tests**
   - Core API functionality
   - Content type detection
   - Data transformation

2. **Integration Tests**
   - Full API endpoint testing
   - Store interface integration

3. **UI Testing**
   - Browser compatibility
   - Responsive design
   - User workflow validation

---

This specification provides a comprehensive blueprint for developing the Store Visualizer for Theater runtime. Implementation should follow the phased approach outlined above, starting with core functionality and progressively adding more advanced features.
