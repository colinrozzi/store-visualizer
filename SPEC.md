# Theater Runtime Store Visualizer Specification

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
   - Content size visualization (e.g., bar chart of largest items)
   - Label relationship visualization (showing which content has multiple labels)
   - Storage usage statistics and trends

## 3. Interface Alignment

The store visualizer directly uses the Theater store interface defined in `store.wit`, which provides the following key operations:

1. **Content Operations**
   - `store`: Store content and retrieve content reference by hash
   - `get`: Retrieve content by content reference
   - `exists`: Check if specific content exists
   - `list-all-content`: List all content references in the store
   - `calculate-total-size`: Get total size of all content

2. **Label Operations**
   - `label`: Attach a label to content
   - `get-by-label`: Get content references for a specific label
   - `remove-label`: Remove a label entirely
   - `remove-from-label`: Remove specific content from a label
   - `put-at-label`: Store content and immediately attach a label
   - `replace-content-at-label`: Replace content at a label
   - `replace-at-label`: Replace content at a label with existing content
   - `list-labels`: List all available labels

## 4. Technical Architecture

### 4.1. Actor Structure

The store-visualizer will be implemented as a Theater WebAssembly actor with:

1. **HTTP Server Component**
   - Serves the web UI (HTML, CSS, JavaScript)
   - Provides REST API endpoints for store operations

2. **Interface Implementation**
   - Implements the Theater actor interface
   - Connects to the runtime for store access
   - Handles initialization with store configuration

3. **Core Components**
   - State management for store connection
   - HTTP request/response handling
   - Content processing and formatting

### 4.2. Web API Structure

The visualizer's HTTP API directly maps to the store operations:

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

### 4.3. User Interface

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

## 5. Data Types

Based on the store.wit interface, we'll use these data types:

```rust
// Content reference - matches the WIT interface
struct ContentRef {
    hash: String,
}
```

For the UI, we'll extend this with derived properties:

```rust
// UI representation with additional metadata
struct ContentItem {
    hash: String,
    size: usize,
    preview: String,      // Content preview/summary
    content_type: String, // Detected content type
    labels: Vec<String>,  // Associated labels
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
    content_by_type: HashMap<String, usize>, // Content type distribution
}
```

## 6. Implementation Requirements

### 6.1. Technology Stack

1. **Backend**
   - Rust with WebAssembly compilation target
   - Theater actor model and interfaces
   - JSON serialization for API communication

2. **Frontend**
   - HTML, CSS, and JavaScript (vanilla or minimal framework)
   - Fetch API for AJAX communication
   - Minimalist UI with responsive design

### 6.2. Development Approach

1. **Phased Implementation**
   - Phase 1: Basic content exploration and viewing
   - Phase 2: Label management and content filtering
   - Phase 3: Advanced visualizations and statistics
   - Phase 4: Content editing and management features

2. **Testing Strategy**
   - Unit tests for core logic
   - Integration tests with mock store
   - End-to-end tests with Theater runtime

### 6.3. Deployment

1. **Actor Manifest**
   - Define required interfaces (HTTP server, runtime, store)
   - Configure port and file system access
   - Specify initialization parameters

2. **Assets**
   - Package HTML, CSS, and JavaScript in the assets directory
   - Initialize with default configuration

## 7. User Workflows

### 7.1. Browsing Content
1. User opens the visualizer web interface
2. Dashboard shows statistics and recent content
3. User can browse all content or filter by criteria
4. User selects content to view details
5. Content is displayed in appropriate format (JSON, text, etc.)

### 7.2. Managing Labels
1. User navigates to Labels section
2. Views existing labels and associated content
3. Can create new labels or edit existing ones
4. Can associate/disassociate content with labels
5. Can search for content by label

### 7.3. Adding/Editing Content
1. User can add new content via upload or direct input
2. Content is stored and displayed with generated hash
3. User can assign labels to the new content
4. User can replace content at existing labels

## 8. Performance Considerations

1. **Pagination and Lazy Loading**
   - Implement pagination for content listing
   - Lazy load content details when viewing
   - Cache frequently accessed content

2. **Content Size Limits**
   - Handle large content appropriately
   - Provide preview for large content rather than full loading
   - Implement download options for large content

3. **Asynchronous Operations**
   - Provide feedback for long-running operations
   - Implement request cancelation for user experience

## 9. Security Considerations

1. **Content Validation**
   - Validate and sanitize all user inputs
   - Implement content size limits for uploads
   - Detect and handle malicious content appropriately

2. **Error Handling**
   - Graceful error handling and informative messages
   - Proper logging of errors for debugging
   - Rate limiting for API endpoints

## 10. Example API Usage

**Getting all content:**
```javascript
// Frontend JavaScript
async function getAllContent() {
  const response = await fetch('/api/content');
  const contentList = await response.json();
  return contentList;
}
```

**Getting content by hash:**
```javascript
async function getContent(hash) {
  const response = await fetch(`/api/content/${hash}`);
  const content = await response.json();
  return content;
}
```

**Adding a label to content:**
```javascript
async function labelContent(label, hash) {
  const response = await fetch(`/api/labels/${label}/content/${hash}`, {
    method: 'PUT'
  });
  return response.ok;
}
```

## 11. Future Extensions

1. **Advanced Visualization**
   - Graph visualization of label relationships
   - Time-series visualization of store growth

2. **Content Analysis**
   - Automatic content type detection
   - Content similarity analysis
   - Duplicate detection

3. **Integration with Other Actors**
   - Visualize relationships between actors and stored content
   - Show which actors are accessing specific content

4. **Export/Import Functionality**
   - Export selected content or labels
   - Import content with custom labels

## 12. Implementation Plan

1. **Sprint 1: Core Infrastructure (1-2 weeks)**
   - Set up project structure
   - Implement actor initialization
   - Create basic HTTP server
   - Connect to store interface

2. **Sprint 2: Content Explorer (1-2 weeks)**
   - Implement content listing
   - Content detail view
   - Basic content operations
   - Frontend UI for content browsing

3. **Sprint 3: Label Management (1-2 weeks)**
   - Implement label operations
   - Label browsing UI
   - Label-content associations
   - Search and filtering

4. **Sprint 4: Advanced Features (1-2 weeks)**
   - Statistics and visualizations
   - Content type detection
   - Performance optimizations
   - Documentation and testing

This specification provides a comprehensive blueprint for developing a new store-visualizer that integrates with the Theater runtime's content store, following the exact interface defined in store.wit.
