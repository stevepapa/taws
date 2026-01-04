# Contributing to taws

Thank you for your interest in contributing to taws! This document provides guidelines and information for contributors.

## Before You Start

**Important:** Before adding a new AWS service or major feature, please start a discussion in our [GitHub Discussions](https://github.com/huseyinbabal/taws/discussions) board. This helps us:

- Avoid duplicate work
- Discuss the best approach
- Ensure the feature aligns with project goals
- Get community feedback

## How to Contribute

1. **Fork the repository**
2. **Create your feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add some amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/taws.git
cd taws

# Build the project
cargo build

# Run in development mode
cargo run

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Architecture

taws follows a data-driven architecture where AWS resource definitions are stored as JSON configuration files. This makes it easy to add new resource types without writing extensive code.

```
src/
├── resources/          # JSON resource definitions (one per service)
│   ├── ec2.json
│   ├── lambda.json
│   ├── s3.json
│   └── ...
├── resource/
│   ├── registry.rs     # Resource registry and loading
│   ├── fetcher.rs      # Generic resource fetcher
│   └── sdk_dispatch.rs # HTTP-based AWS API calls
├── aws/
│   ├── client.rs       # AWS HTTP client management
│   ├── credentials.rs  # Credential loading (profiles, env vars)
│   ├── http.rs         # Lightweight HTTP client with SigV4 signing
│   └── profiles.rs     # AWS profile handling
└── ui/
    ├── table.rs        # Resource table view
    ├── details.rs      # Resource details view
    └── ...
```

### Lightweight Design

taws uses a custom lightweight HTTP client with AWS SigV4 signing instead of the full AWS SDK. This results in:

- **Fast builds** - ~100 dependencies vs ~500+ with full SDK
- **Small binary** - ~5MB release binary
- **Quick compilation** - Seconds instead of minutes

## Adding a New AWS Service

To add support for a new AWS service, follow these steps:

### 1. Start a Discussion

Before writing any code, [open a discussion](https://github.com/huseyinbabal/taws/discussions/new?category=ideas) to propose the new service. Include:

- Which AWS service you want to add
- Which resources/operations you plan to support
- Why this service would be valuable

### 2. Add the Service Definition

Add the AWS service definition to `src/aws/http.rs`:

```rust
"myservice" => Some(ServiceDefinition {
    signing_name: "myservice",
    endpoint_prefix: "myservice",
    api_version: "2023-01-01",
    protocol: Protocol::Json,  // or Query, RestJson, RestXml
    target_prefix: Some("MyService"),  // for JSON protocol
    is_global: false,
}),
```

### 3. Add Resource JSON Definition

Create `src/resources/myservice.json`:

```json
{
  "resources": {
    "myservice-items": {
      "display_name": "MyService Items",
      "service": "myservice",
      "sdk_method": "list_items",
      "response_path": "items",
      "id_field": "ItemId",
      "name_field": "ItemName",
      "is_global": false,
      "columns": [
        { "header": "ID", "json_path": "ItemId", "width": 20 },
        { "header": "Name", "json_path": "ItemName", "width": 30 },
        { "header": "Status", "json_path": "Status", "width": 15, "color_map": "status" }
      ]
    }
  }
}
```

### 4. Add SDK Dispatch Handler

Add the HTTP dispatch handler to `src/resource/sdk_dispatch.rs`:

```rust
("myservice", "list_items") => {
    let response = clients.http.json_request(
        "myservice",
        "ListItems",
        "{}"
    ).await?;
    
    let items = response.get("Items")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    
    Ok(json!({ "items": items }))
}
```

### 5. Test Your Changes

```bash
# Build and run
cargo run

# Test the new resource
# Press : and type your resource name
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write descriptive commit messages
- Add comments for complex logic

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update documentation if needed
- Ensure all tests pass
- Reference any related issues or discussions

## Questions?

If you have questions, feel free to:

- Open a [Discussion](https://github.com/huseyinbabal/taws/discussions)
- Check existing issues and PRs

Thank you for contributing!
