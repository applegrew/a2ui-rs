use a2ui_types::common::SpecVersion;
use a2ui_types::v09::catalog::Catalog;
use a2ui_types::v10::catalog::Catalog as V10Catalog;

/// Context to inject into an LLM system prompt so that it can generate valid A2UI messages.
#[derive(Debug, Clone)]
pub struct PromptContext {
    /// The spec version being used.
    pub spec_version: SpecVersion,

    /// The catalog schema as a JSON string (for the LLM to reference).
    pub catalog_schema_json: String,

    /// Protocol description and rules text for the LLM.
    pub protocol_description: String,

    /// Example messages demonstrating valid A2UI output.
    pub examples: String,
}

impl std::fmt::Display for PromptContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n## Catalog Schema\n\n```json\n{}\n```\n\n## Examples\n\n{}",
            self.protocol_description, self.catalog_schema_json, self.examples
        )
    }
}

/// Build a prompt context for a v0.9-family catalog, defaulting to protocol version v0.9.1.
///
/// Generates a `PromptContext` containing the protocol description, catalog schema,
/// and example messages that can be injected into an LLM system prompt.
///
/// # Arguments
/// * `catalog` — The catalog to build the prompt context for.
/// * `catalog_schema` — The raw JSON Schema of the catalog (for inclusion in prompt).
pub fn build_prompt_context_v09(
    catalog: &Catalog,
    catalog_schema: &serde_json::Value,
) -> PromptContext {
    build_prompt_context_v09_with_version(catalog, catalog_schema, SpecVersion::V0_9_1)
}

/// Build a prompt context for a v0.9-family catalog with an explicit protocol version.
///
/// # Arguments
/// * `catalog` — The catalog to build the prompt context for.
/// * `catalog_schema` — The raw JSON Schema of the catalog (for inclusion in prompt).
/// * `spec_version` — The protocol version (`V0_9` or `V0_9_1`) to describe and exemplify.
pub fn build_prompt_context_v09_with_version(
    catalog: &Catalog,
    catalog_schema: &serde_json::Value,
    spec_version: SpecVersion,
) -> PromptContext {
    let catalog_schema_json =
        serde_json::to_string_pretty(catalog_schema).unwrap_or_else(|_| "{}".to_string());

    let version = spec_version.to_string();

    let protocol_description = format!(
        r#"You are generating UI using the A2UI {version} protocol. A2UI is a JSON-based streaming protocol for agent-driven user interfaces.

## Protocol Rules

1. Every message must be a JSON object with a "version" field set to "{version}".
2. Each message must contain exactly ONE of: "createSurface", "updateComponents", "updateDataModel", or "deleteSurface".
3. A surface must be created with "createSurface" before sending "updateComponents" or "updateDataModel" to it.
4. Components use a flat adjacency list model — each component has a unique "id" and references children by their IDs.
5. One component must have id "root" to serve as the root of the component tree.
6. The "component" field specifies the component type (e.g., "Text", "Button", "Column").
7. Component properties are defined inline alongside "id" and "component".
8. Data bindings use JSON Pointer paths (e.g., "/user/name") via the "path" property in a binding object.
9. You must only use components and functions defined in the catalog below.

## Catalog: {catalog_id}"#,
        version = version,
        catalog_id = catalog.catalog_id
    );

    let examples = format!(
        r#"### Example: Create a surface and add components

```json
{{"version": "{version}", "createSurface": {{"surfaceId": "my_surface", "catalogId": "CATALOG_ID_HERE"}}}}
```

```json
{{"version": "{version}", "updateComponents": {{"surfaceId": "my_surface", "components": [{{"id": "root", "component": "Column", "children": ["greeting"]}}, {{"id": "greeting", "component": "Text", "text": "Hello, world!"}}]}}}}
```

### Example: Update the data model

```json
{{"version": "{version}", "updateDataModel": {{"surfaceId": "my_surface", "path": "/user/name", "value": "Alice"}}}}
```

### Example: Delete a surface

```json
{{"version": "{version}", "deleteSurface": {{"surfaceId": "my_surface"}}}}
```
"#
    );

    PromptContext {
        spec_version,
        catalog_schema_json,
        protocol_description,
        examples,
    }
}

/// Build a prompt context for a v1.0 catalog.
///
/// Generates a `PromptContext` containing the protocol description, catalog schema,
/// and example messages that can be injected into an LLM system prompt.
///
/// # Arguments
/// * `catalog` — The catalog to build the prompt context for.
/// * `catalog_schema` — The raw JSON Schema of the catalog (for inclusion in prompt).
pub fn build_prompt_context_v10(
    catalog: &V10Catalog,
    catalog_schema: &serde_json::Value,
) -> PromptContext {
    let catalog_schema_json =
        serde_json::to_string_pretty(catalog_schema).unwrap_or_else(|_| "{}".to_string());

    let catalog_id = catalog.catalog_id.as_str();

    let protocol_description = format!(
        r#"You are generating UI using the A2UI v1.0 protocol. A2UI is a JSON-based streaming protocol for agent-driven user interfaces.

## Protocol Rules

1. Every message must be a JSON object with a "version" field set to "v1.0".
2. Each message must contain exactly ONE of: "createSurface", "updateComponents", "updateDataModel", "deleteSurface", "callRendererFunction", or "agentFunctionResponse".
3. A surface must be created with "createSurface" before sending "updateComponents" or "updateDataModel" to it.
4. Components use a flat adjacency list model — each component has a unique "id" and references children by their IDs.
5. One component must have id "root" to serve as the root of the component tree.
6. The "component" field specifies the component type (e.g., "Text", "Button", "Column").
7. Component properties are defined inline alongside "id" and "component".
8. Data bindings use JSON Pointer paths via the reserved "@path" key (e.g., {{"@path": "/user/name"}}).
9. Function calls use the reserved "@call" key (e.g., {{"@call": "email", "args": {{...}}}}).
10. "createSurface" may optionally embed an initial "components" list and "dataModel" object.
11. You must only use components and functions defined in the catalog below.

## Catalog: {catalog_id}"#
    );

    let examples = r#"### Example: Create a surface with an embedded component tree

```json
{"version": "v1.0", "createSurface": {"surfaceId": "my_surface", "catalogId": "CATALOG_ID_HERE", "components": [{"id": "root", "component": "Column", "children": ["greeting"]}, {"id": "greeting", "component": "Text", "text": {"@path": "/greeting"}}], "dataModel": {"greeting": "Hello, world!"}}}
```

### Example: Update components

```json
{"version": "v1.0", "updateComponents": {"surfaceId": "my_surface", "components": [{"id": "root", "component": "Column", "children": ["greeting"]}, {"id": "greeting", "component": "Text", "text": "Hello, world!"}]}}
```

### Example: Update the data model

```json
{"version": "v1.0", "updateDataModel": {"surfaceId": "my_surface", "path": "/user/name", "value": "Alice"}}
```

### Example: Delete a surface

```json
{"version": "v1.0", "deleteSurface": {"surfaceId": "my_surface"}}
```
"#
    .to_string();

    PromptContext {
        spec_version: SpecVersion::V1_0,
        catalog_schema_json,
        protocol_description,
        examples,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_build_prompt_context_v09() {
        let catalog = Catalog {
            catalog_id: "https://example.com/basic.json".to_string(),
            components: Some(json!({"Text": {"type": "object"}})),
            functions: None,
            theme: None,
        };
        let schema = json!({"type": "object"});

        let ctx = build_prompt_context_v09(&catalog, &schema);
        assert_eq!(ctx.spec_version, SpecVersion::V0_9_1);
        assert!(ctx.protocol_description.contains("v0.9.1"));
        assert!(ctx
            .protocol_description
            .contains("https://example.com/basic.json"));
        assert!(ctx.catalog_schema_json.contains("object"));
        assert!(ctx.examples.contains("createSurface"));
    }

    #[test]
    fn test_build_prompt_context_v09_explicit_v09() {
        let catalog = Catalog {
            catalog_id: "https://example.com/basic.json".to_string(),
            components: None,
            functions: None,
            theme: None,
        };
        let schema = json!({"type": "object"});

        let ctx = build_prompt_context_v09_with_version(&catalog, &schema, SpecVersion::V0_9);
        assert_eq!(ctx.spec_version, SpecVersion::V0_9);
        assert!(ctx.protocol_description.contains("v0.9"));
        assert!(!ctx.protocol_description.contains("v0.9.1"));
    }

    #[test]
    fn test_prompt_context_display() {
        let catalog = Catalog {
            catalog_id: "test".to_string(),
            components: None,
            functions: None,
            theme: None,
        };
        let schema = json!({"type": "object"});
        let ctx = build_prompt_context_v09(&catalog, &schema);
        let display = format!("{}", ctx);
        assert!(display.contains("Catalog Schema"));
        assert!(display.contains("Examples"));
    }

    #[test]
    fn test_build_prompt_context_v10() {
        let catalog = V10Catalog {
            catalog_id: "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json".to_string(),
            instructions: None,
            components: None,
            functions: None,
            protocol_version: Some("1.0".to_string()),
        };
        let schema = json!({"type": "object"});

        let ctx = build_prompt_context_v10(&catalog, &schema);
        assert_eq!(ctx.spec_version, SpecVersion::V1_0);
        assert!(ctx.protocol_description.contains("v1.0"));
        assert!(ctx.protocol_description.contains("callRendererFunction"));
        assert!(ctx.examples.contains("dataModel"));
        assert!(ctx.examples.contains("@path"));
    }
}
