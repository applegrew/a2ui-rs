# a2ui-rs

A runtime-agnostic, **stateless** Rust toolkit implementing the [A2UI protocol](https://a2ui.org) (v0.9, v0.9.1, and v1.0) for server-side agent applications. It provides typed message models, catalog negotiation, LLM prompt generation, JSON Schema validation, and message builders — all as pure functions and minimal traits.

**All session management, agent memory, and conversation state are owned by the agent harness (e.g. [Rig](https://github.com/0xPlaygrounds/rig)), not this library.**

> **Note:** v1.0 is the current primary target. The v0.9-family modules (`v09`) are retained for compatibility; message builders default to emitting `"v0.9.1"` (the current production release).

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│              Agent Harness (Rig etc.)                        │
│  Owns: LLM loop, session state, memory, surface tracking     │
│  Uses a2ui-rs to:                                            │
│  - Build system prompt context (catalog + protocol schema)   │
│  - Construct typed server-to-client messages                 │
│  - Validate LLM output before sending to client              │
│  - Parse incoming client-to-server messages                  │
│  - Negotiate catalogs with client capabilities               │
│  - Send validated messages via ClientTransport trait         │
└──────┬──────────────────┬──────────────────┬─────────────────┘
       │                  │                  │
       ▼                  ▼                  ▼
   CatalogProvider    a2ui-core         ClientTransport
   (trait impl by     (pure functions   (trait impl by
    harness/plugin)    + utilities)      harness/plugin)
```

The library is a **toolkit**, not an engine. The harness calls into it; it never drives control flow.

## Crate Structure

| Crate | Description |
|-------|-------------|
| **`a2ui-types`** | Serde-based typed models for v0.9, v0.9.1, and v1.0 messages, capabilities, catalogs, and data binding types. Pure data — no logic, no async. |
| **`a2ui-core`** | Traits (`ClientTransport`, `CatalogProvider`), pure functions for negotiation, validation, prompt building, message construction, and parsing. |
| **`a2ui-validation`** | JSON Schema validation engine. Validates outgoing messages against catalog schemas and returns structured `ValidationError`s matching the spec's `VALIDATION_FAILED` format. |

### Module Map

```
a2ui-types/src/
├── common.rs           # SurfaceId, ComponentId, CatalogId, SpecVersion
├── v09/
│   ├── server_to_client.rs   # createSurface, updateComponents, updateDataModel, deleteSurface
│   ├── client_to_server.rs   # action, error (VALIDATION_FAILED)
│   ├── capabilities.rs       # ClientCapabilities, ServerCapabilities
│   ├── client_data_model.rs  # ClientDataModel (sendDataModel sync)
│   ├── common_types.rs       # DynamicString/Number/Boolean, ChildList, Action, FunctionCall, CheckRule
│   └── catalog.rs            # Catalog, FunctionDefinition
└── v10/
    ├── agent_to_renderer.rs  # createSurface, updateComponents, updateDataModel, deleteSurface,
    │                         #   callRendererFunction, agentFunctionResponse
    ├── renderer_to_agent.rs  # action, callAgentFunction, rendererFunctionResponse, error
    ├── capabilities.rs       # RendererCapabilities, AgentCapabilities
    ├── common_types.rs       # DataBinding (@path), FunctionCall (@call), Dynamic*,
    │                         #   ChildList, Action, CheckRule, FunctionResponse, ValidationResult
    └── catalog.rs            # Catalog

a2ui-core/src/
├── traits.rs          # ClientTransport, CatalogProvider
├── negotiation.rs     # negotiate_catalog(), negotiate_catalog_with_inline()
├── validation.rs      # validate_message(), parse_client_message_*(), serialize_message_*()
├── prompt.rs          # build_prompt_context_v09*(), build_prompt_context_v10()
├── message.rs         # v0.9 and v1.0 message builders
└── error.rs           # A2uiError

a2ui-validation/src/
└── schema_validator.rs  # SchemaValidator, validate()
```

## Quick Start (v0.9 / v0.9.1)

### 1. Implement the Traits

```rust
use a2ui_core::traits::{CatalogProvider, CatalogInfo, ClientTransport};
use a2ui_types::common::CatalogId;
use a2ui_types::v09::catalog::Catalog;
use async_trait::async_trait;

struct MyCatalogProvider { /* ... */ }

impl CatalogProvider for MyCatalogProvider {
    fn available_catalogs(&self) -> Vec<CatalogInfo> {
        vec![CatalogInfo {
            catalog_id: CatalogId::new("https://a2ui.org/specification/v0_9_1/basic_catalog.json"),
            description: Some("Basic A2UI catalog".to_string()),
        }]
    }

    fn get_catalog(&self, id: &CatalogId) -> Option<Catalog> {
        // Return the full typed catalog definition
        todo!()
    }

    fn get_catalog_schema(&self, id: &CatalogId) -> Option<serde_json::Value> {
        // Return the raw JSON Schema for validation / prompt injection
        todo!()
    }
}

struct MyTransport { /* e.g. WebSocket sender */ }

#[async_trait]
impl ClientTransport for MyTransport {
    async fn send_to_client(&self, msg: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send bytes over your transport (WebSocket, SSE, A2A, etc.)
        todo!()
    }
}
```

### 2. Negotiate a Catalog

```rust
use a2ui_core::negotiation::negotiate_catalog;

// Client sends its supported catalog IDs (e.g. from a2uiClientCapabilities)
let client_supported = vec![
    "https://a2ui.org/specification/v0_9_1/basic_catalog.json".to_string(),
];

let result = negotiate_catalog(&client_supported, &my_catalog_provider)?;
println!("Using catalog: {}", result.catalog_id.as_str());
```

### 3. Build LLM Prompt Context

```rust
use a2ui_core::prompt::build_prompt_context_v09;

let catalog = my_catalog_provider.get_catalog(&result.catalog_id).unwrap();
let schema = my_catalog_provider.get_catalog_schema(&result.catalog_id).unwrap();

// Defaults to protocol version v0.9.1
let prompt_ctx = build_prompt_context_v09(&catalog, &schema);

// Inject into your LLM system prompt
let system_prompt = format!(
    "You are a helpful assistant.\n\n{}",
    prompt_ctx  // Implements Display
);
```

### 4. Construct & Validate Messages

```rust
use a2ui_core::message::*;
use a2ui_core::validation::*;
use serde_json::json;

// Build a createSurface message (emits "v0.9.1" by default)
let create_msg = CreateSurfaceBuilder::new("booking-surface", result.catalog_id.as_str())
    .send_data_model(true)
    .build();

// Build a component update
let update_msg = UpdateComponentsBuilder::new("booking-surface")
    .add_component(json!({"id": "root", "component": "Column", "children": ["greeting"]}))
    .add_component(json!({"id": "greeting", "component": "Text", "text": "Hello!"}))
    .build();

// Validate before sending
let msg_json = serde_json::to_value(&create_msg)?;
validate_message(&msg_json, &schema, "booking-surface")?;
```

### 5. Serialize & Send

```rust
use a2ui_core::validation::serialize_message_v09;

let bytes = serialize_message_v09(&create_msg)?;
my_transport.send_to_client(&bytes).await?;
```

### 6. Parse Incoming Client Messages

```rust
use a2ui_core::validation::{parse_client_message_auto, ParsedClientMessage};

let parsed = parse_client_message_auto(&raw_bytes)?;

match parsed {
    ParsedClientMessage::V09(msg) => {
        if let Some(action) = msg.action {
            println!("Action: {} on surface {}", action.name, action.surface_id.as_str());
            // Route to the appropriate handler based on action.name
        }
        if let Some(err) = msg.error {
            eprintln!("Client error: {} - {}", err.code, err.message);
            // Feed back to LLM for self-correction
        }
    }
    ParsedClientMessage::V10(msg) => {
        if let Some(action) = msg.action {
            println!("v1.0 action: {} on surface {}", action.name, action.surface_id.as_str());
        }
    }
}
```

## Quick Start (v1.0)

The v1.0 API lives under `a2ui_types::v10` and uses the reserved `@path` / `@call`
keys for data bindings and function calls.

```rust
use a2ui_core::message::*;
use a2ui_core::validation::serialize_message_v10;
use a2ui_types::v10::common_types::FunctionCall;
use serde_json::json;

// createSurface can embed the initial component tree and data model
let create_msg = CreateSurfaceBuilderV10::new("booking-surface")
    .catalog_id("https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json")
    .add_component(json!({"id": "root", "component": "Column", "children": ["greeting"]}))
    .add_component(json!({"id": "greeting", "component": "Text", "text": {"@path": "/greeting"}}))
    .data_model(json!({"greeting": "Hello, world!"}))
    .build();

let bytes = serialize_message_v10(&create_msg)?;

// Execute a function on the renderer
let call = FunctionCall {
    call: "getScreenResolution".to_string(),
    catalog_id: Some("https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json".to_string()),
    args: Some(json!({"screenIndex": 0})),
};
let call_msg = call_renderer_function_v10("call_123", call);
```

## Handle Validation Errors

When the LLM generates invalid A2UI JSON, the validation functions return structured errors that can be fed back to the LLM:

```rust
use a2ui_core::error::A2uiError;

match validate_message(&llm_output_json, &catalog_schema, "my-surface") {
    Ok(()) => {
        // Message is valid — serialize and send
    }
    Err(A2uiError::Validation(errors)) => {
        // Feed errors back to LLM for self-correction
        for err in &errors {
            eprintln!("[{}] {}: {} at {}", err.code, err.surface_id, err.message, err.path);
        }
        // Re-prompt the LLM with the error details
    }
    Err(other) => {
        eprintln!("Unexpected error: {}", other);
    }
}
```

## Spec Version Support Matrix

| Feature | v0.9 | v0.9.1 | v1.0 |
|---------|------|--------|------|
| Surface creation | `createSurface` | `createSurface` | `createSurface` (can embed `components` + `dataModel`) |
| Component updates | `updateComponents` | `updateComponents` | `updateComponents` |
| Data model updates | `updateDataModel` | `updateDataModel` | `updateDataModel` (`value` required) |
| Surface deletion | `deleteSurface` | `deleteSurface` | `deleteSurface` |
| Client actions | `action` | `action` | `action` (adds `userMessage`, `metadata`) |
| Function calls | `FunctionCall` (`call`) | `FunctionCall` (`call`) | `FunctionCall` (`@call`); bidirectional `callRendererFunction` / `callAgentFunction` |
| Data binding | `path` | `path` | `@path` (reserved `@` namespace) |
| Check rules | `CheckRule` | `CheckRule` | `CheckRule` (structured `ValidationResult`) |
| Data model sync | `sendDataModel` | `sendDataModel` | `sendDataModel` |
| Capabilities | `ClientCapabilities` / `ServerCapabilities` | same | `RendererCapabilities` / `AgentCapabilities` |
| Version field | `"v0.9"` | `"v0.9"` or `"v0.9.1"` | `"v1.0"` |
| Catalog negotiation | ✓ | ✓ | ✓ (mixable catalogs) |
| Message validation | ✓ | ✓ | ✓ |
| Prompt generation | ✓ | ✓ | ✓ |

## Design Philosophy

- **Stateless toolkit**: No session management, no surface tracking, no data model storage. The harness owns all state and calls library functions as needed.
- **Runtime-agnostic**: Only the `ClientTransport` trait is async (via `async-trait`). Everything else is synchronous pure functions. No tokio/async-std dependency in the library itself.
- **Separate type hierarchies**: `v09::*` and `v10::*` are distinct types. The harness chooses which module to use. No version-polymorphism overhead.
- **Catalog as typed + raw JSON**: `Catalog` structs for negotiation metadata; raw `serde_json::Value` for validation and LLM prompt injection.
- **Spec-compatible error reporting**: `ValidationError` mirrors the A2UI `VALIDATION_FAILED` format so the harness can forward errors directly to the LLM or client.

### v1.0 validation note

The official v1.0 envelope schemas (`agent_to_renderer.json`, `renderer_to_agent.json`)
reference `common_types.json` and `catalog.json` via relative `$ref`. `a2ui_validation`
validates against whatever self-contained schema the harness supplies (it does not
bundle or resolve the official spec files). To validate v1.0 messages against the full
envelope schema, pass a schema that has been bundled/resolved (e.g. via a JSON Schema
bundler) or a catalog-specific schema. The `jsonschema` backend supports JSON Schema
draft 2020-12 (including `unevaluatedProperties`).

## TODO

- **Typed v1.0 `CatalogProvider`**: The `CatalogProvider` trait's `get_catalog` still
  returns the v0.9 `a2ui_types::v09::catalog::Catalog` type. v1.0 negotiation and
  prompt building don't depend on it (they use catalog IDs and raw schemas), but a
  fully v1.0-typed provider is not yet available. Add a v1.0 catalog-returning trait
  (or make `CatalogProvider` version-agnostic) so v1.0 harnesses can supply a typed
  `a2ui_types::v10::catalog::Catalog`.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `serde` + `serde_json` | Serialization / deserialization |
| `async-trait` | Async trait support for `ClientTransport` |
| `thiserror` | Ergonomic error types |
| `jsonschema` | JSON Schema validation engine |

## Building & Testing

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p a2ui-types
cargo test -p a2ui-core
cargo test -p a2ui-validation
```

## License

MIT
