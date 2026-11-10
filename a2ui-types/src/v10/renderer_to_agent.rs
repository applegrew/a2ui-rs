use serde::{Deserialize, Serialize};

use crate::common::SurfaceId;

use super::common_types::{FunctionCall, FunctionResponse};

/// A single renderer-to-agent message in v1.0.
///
/// Must contain `version` and exactly one of `action`, `callAgentFunction`,
/// `rendererFunctionResponse`, or `error`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RendererToAgentMessage {
    /// Protocol version — always `"v1.0"`.
    pub version: String,

    /// Reports a user-initiated action from a component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,

    /// Signals the agent to execute a function remotely on behalf of the renderer.
    #[serde(rename = "callAgentFunction", skip_serializing_if = "Option::is_none")]
    pub call_agent_function: Option<CallAgentFunction>,

    /// The renderer's response to an agent-initiated `callRendererFunction`.
    #[serde(
        rename = "rendererFunctionResponse",
        skip_serializing_if = "Option::is_none"
    )]
    pub renderer_function_response: Option<FunctionResponse>,

    /// Reports a renderer-side error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RendererError>,
}

/// A user-initiated action event in v1.0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    /// The name of the action.
    pub name: String,

    /// Optional human-readable description of the action performed.
    #[serde(rename = "userMessage", skip_serializing_if = "Option::is_none")]
    pub user_message: Option<String>,

    /// The ID of the surface where the action originated.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,

    /// The ID of the component that triggered the action.
    #[serde(rename = "sourceComponentId")]
    pub source_component_id: String,

    /// An ISO 8601 timestamp of when the event occurred.
    pub timestamp: String,

    /// Key-value pairs from the component's action context, after resolving data bindings.
    pub context: serde_json::Value,

    /// Optional renderer-side metadata to send back with the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// A renderer-initiated request to execute a function on the agent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallAgentFunction {
    /// The surface ID where the call originated.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,

    /// A unique identifier for this invocation instance.
    #[serde(rename = "functionCallId")]
    pub function_call_id: String,

    /// The description of the function call.
    #[serde(rename = "callFunction")]
    pub call_function: FunctionCall,
}

/// A renderer-side error report.
///
/// Either a structured validation error (with `surfaceId` and `path`) or a
/// generic error (optionally carrying a `functionCallId`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RendererError {
    ValidationFailed {
        code: String,
        #[serde(rename = "surfaceId")]
        surface_id: SurfaceId,
        path: String,
        message: String,
    },
    Generic {
        code: String,
        message: String,
        #[serde(rename = "surfaceId", skip_serializing_if = "Option::is_none")]
        surface_id: Option<SurfaceId>,
        #[serde(rename = "functionCallId", skip_serializing_if = "Option::is_none")]
        function_call_id: Option<String>,
    },
}
