# Store Visualizer

A Theater actor that provides a web interface for visualizing the contents of a key-value store.

## Features

- Connects to an existing key-value store actor
- Displays all stored entries in a clean web interface
- Real-time search/filtering of entries
- Detailed JSON view of entry contents
- Support for both JSON and binary data viewing

## Setup

1. Build the actor:
```bash
cargo build --release
```

2. Update the init.json with your key-value store actor ID:
```json
{
    "key_value_actor": "your-key-value-actor-id"
}
```

3. Start the actor using Theater

## Usage

The web interface will be available at `http://localhost:8085` and provides:

- A list view of all entries in the store
- Search functionality to filter entries by key
- Detailed view of entry contents
- Refresh button to get latest store contents

## Architecture

The actor:
1. Connects to a key-value store actor on initialization
2. Provides a web interface for visualization
3. Makes "All" requests to the store to fetch contents
4. Renders both JSON and binary data appropriately