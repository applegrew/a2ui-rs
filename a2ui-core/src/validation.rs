use a2ui_types::common::SpecVersion;

use crate::error::A2uiError;

/// Validate an outgoing server-to-client message against a catalog schema.
///
/// This performs pre-send validation as recommended by the A2UI spec
/// (agent-side, before transmitting to the client).
///
/// # Arguments
/// * `message` — The serialized message as a `serde_json::Value`.
/// * `catalog_schema` — The JSON Schema of the catalog the surface uses.
/// * `surface_id` — The surface ID for error reporting.
///
/// # Returns
/// `Ok(())` if the message is valid, or `Err(A2uiError::Validation(...))` with structured errors.
pub fn validate_message(
    message: &serde_json::Value,
    catalog_schema: &serde_json::Value,
    surface_id: &str,
) -> Result<(), A2uiError> {
    a2ui_validation::validate(catalog_schema, message, surface_id).map_err(A2uiError::Validation)
}

/// Parse and validate an incoming client-to-server message (v0.9).
///
/// Deserializes raw JSON into the typed `ClientToServerMessage` and optionally
/// validates it.
///
/// # Arguments
/// * `raw_json` — The raw JSON bytes from the client.
///
/// # Returns
/// The parsed message, or a serialization error.
pub fn parse_client_message_v09(
    raw_json: &[u8],
) -> Result<a2ui_types::v09::client_to_server::ClientToServerMessage, A2uiError> {
    serde_json::from_slice(raw_json).map_err(A2uiError::Serialization)
}

/// Parse and validate an incoming renderer-to-agent message (v1.0).
///
/// # Arguments
/// * `raw_json` — The raw JSON bytes from the renderer.
///
/// # Returns
/// The parsed message, or a serialization error.
pub fn parse_client_message_v10(
    raw_json: &[u8],
) -> Result<a2ui_types::v10::renderer_to_agent::RendererToAgentMessage, A2uiError> {
    serde_json::from_slice(raw_json).map_err(A2uiError::Serialization)
}

/// Parse an incoming client-to-server message, auto-detecting the version
/// from the JSON content.
///
/// v0.9-family messages carry `"version": "v0.9"` or `"v0.9.1"`; v1.0 messages
/// carry `"version": "v1.0"`.
///
/// # Returns
/// An enum indicating which version was parsed.
pub fn parse_client_message_auto(raw_json: &[u8]) -> Result<ParsedClientMessage, A2uiError> {
    let value: serde_json::Value =
        serde_json::from_slice(raw_json).map_err(A2uiError::Serialization)?;

    let version = value
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| A2uiError::Other("message has no \"version\" field".to_string()))?;

    match version {
        "v0.9" | "v0.9.1" => {
            let msg: a2ui_types::v09::client_to_server::ClientToServerMessage =
                serde_json::from_value(value).map_err(A2uiError::Serialization)?;
            Ok(ParsedClientMessage::V09(msg))
        }
        "v1.0" => {
            let msg: a2ui_types::v10::renderer_to_agent::RendererToAgentMessage =
                serde_json::from_value(value).map_err(A2uiError::Serialization)?;
            Ok(ParsedClientMessage::V10(msg))
        }
        other => Err(A2uiError::Other(format!(
            "unsupported A2UI protocol version: {other}"
        ))),
    }
}

/// A parsed client-to-server message tagged with its version.
///
/// Variants differ in size because v0.9 and v1.0 messages have different shapes;
/// this enum is only constructed during parsing and matched once, so the size
/// difference is acceptable.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedClientMessage {
    V09(a2ui_types::v09::client_to_server::ClientToServerMessage),
    V10(a2ui_types::v10::renderer_to_agent::RendererToAgentMessage),
}

impl ParsedClientMessage {
    /// Returns the spec version of the parsed message.
    pub fn spec_version(&self) -> SpecVersion {
        match self {
            ParsedClientMessage::V09(msg) => {
                SpecVersion::parse(&msg.version).unwrap_or(SpecVersion::V0_9_1)
            }
            ParsedClientMessage::V10(_) => SpecVersion::V1_0,
        }
    }
}

/// Serialize a v0.9 server-to-client message to JSON bytes.
pub fn serialize_message_v09(
    msg: &a2ui_types::v09::server_to_client::ServerToClientMessage,
) -> Result<Vec<u8>, A2uiError> {
    serde_json::to_vec(msg).map_err(A2uiError::Serialization)
}

/// Serialize a v1.0 agent-to-renderer message to JSON bytes.
pub fn serialize_message_v10(
    msg: &a2ui_types::v10::agent_to_renderer::AgentToRendererMessage,
) -> Result<Vec<u8>, A2uiError> {
    serde_json::to_vec(msg).map_err(A2uiError::Serialization)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_v09_action() {
        let raw = serde_json::to_vec(&json!({
            "version": "v0.9",
            "action": {
                "name": "submit",
                "surfaceId": "s1",
                "sourceComponentId": "btn1",
                "timestamp": "2025-01-01T00:00:00Z",
                "context": {}
            }
        }))
        .unwrap();

        let msg = parse_client_message_v09(&raw).unwrap();
        assert!(msg.action.is_some());
        assert_eq!(msg.action.unwrap().name, "submit");
    }

    #[test]
    fn test_parse_auto_detect_v09() {
        let raw = serde_json::to_vec(&json!({
            "version": "v0.9",
            "action": {
                "name": "click",
                "surfaceId": "s1",
                "sourceComponentId": "c1",
                "timestamp": "2025-01-01T00:00:00Z",
                "context": {}
            }
        }))
        .unwrap();

        let result = parse_client_message_auto(&raw).unwrap();
        assert_eq!(result.spec_version(), SpecVersion::V0_9);
    }

    #[test]
    fn test_parse_auto_detect_v09_1() {
        let raw = serde_json::to_vec(&json!({
            "version": "v0.9.1",
            "action": {
                "name": "click",
                "surfaceId": "s1",
                "sourceComponentId": "c1",
                "timestamp": "2025-01-01T00:00:00Z",
                "context": {}
            }
        }))
        .unwrap();

        let result = parse_client_message_auto(&raw).unwrap();
        assert_eq!(result.spec_version(), SpecVersion::V0_9_1);
    }

    #[test]
    fn test_parse_auto_detect_unknown_version() {
        let raw = serde_json::to_vec(&json!({
            "version": "v9.99",
            "action": {
                "name": "click",
                "surfaceId": "s1",
                "sourceComponentId": "c1",
                "timestamp": "2025-01-01T00:00:00Z",
                "context": {}
            }
        }))
        .unwrap();

        let result = parse_client_message_auto(&raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_v09() {
        let msg = a2ui_types::v09::server_to_client::ServerToClientMessage {
            version: "v0.9".to_string(),
            create_surface: Some(a2ui_types::v09::server_to_client::CreateSurface {
                surface_id: "s1".into(),
                catalog_id: "https://example.com/basic.json".to_string(),
                theme: None,
                send_data_model: None,
            }),
            update_components: None,
            update_data_model: None,
            delete_surface: None,
        };

        let bytes = serialize_message_v09(&msg).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["version"], "v0.9");
        assert!(value.get("createSurface").is_some());
    }

    #[test]
    fn test_validate_message_valid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "version": { "const": "v0.9" },
                "createSurface": { "type": "object" }
            },
            "required": ["version"]
        });
        let message = json!({
            "version": "v0.9",
            "createSurface": { "surfaceId": "s1", "catalogId": "c1" }
        });

        assert!(validate_message(&message, &schema, "s1").is_ok());
    }

    #[test]
    fn test_validate_message_invalid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "version": { "const": "v0.9" }
            },
            "required": ["version"]
        });
        let message = json!({ "noversion": true });

        let result = validate_message(&message, &schema, "s1");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_auto_detect_v10() {
        let raw = serde_json::to_vec(&json!({
            "version": "v1.0",
            "action": {
                "name": "click",
                "surfaceId": "s1",
                "sourceComponentId": "c1",
                "timestamp": "2025-01-01T00:00:00Z",
                "context": {}
            }
        }))
        .unwrap();

        let result = parse_client_message_auto(&raw).unwrap();
        assert_eq!(result.spec_version(), SpecVersion::V1_0);

        match result {
            ParsedClientMessage::V10(msg) => {
                let action = msg.action.unwrap();
                assert_eq!(action.name, "click");
            }
            _ => panic!("expected v1.0 message"),
        }
    }

    #[test]
    fn test_parse_v10_call_agent_function() {
        let raw = serde_json::to_vec(&json!({
            "version": "v1.0",
            "callAgentFunction": {
                "surfaceId": "s1",
                "functionCallId": "fc_1",
                "callFunction": {
                    "@call": "verifyProvider",
                    "catalogId": "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json",
                    "args": { "providerId": "PRV-102" }
                }
            }
        }))
        .unwrap();

        let msg = parse_client_message_v10(&raw).unwrap();
        let caf = msg.call_agent_function.unwrap();
        assert_eq!(caf.function_call_id, "fc_1");
        assert_eq!(caf.call_function.call, "verifyProvider");
    }

    #[test]
    fn test_serialize_v10() {
        let msg = a2ui_types::v10::agent_to_renderer::AgentToRendererMessage {
            version: "v1.0".to_string(),
            create_surface: Some(a2ui_types::v10::agent_to_renderer::CreateSurface {
                surface_id: "s1".into(),
                catalog_id: Some("https://example.com/catalog.json".to_string()),
                send_data_model: None,
                components: None,
                data_model: None,
                metadata: None,
            }),
            update_components: None,
            update_data_model: None,
            delete_surface: None,
            call_renderer_function: None,
            agent_function_response: None,
        };

        let bytes = serialize_message_v10(&msg).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["version"], "v1.0");
        assert!(value.get("createSurface").is_some());
    }
}
