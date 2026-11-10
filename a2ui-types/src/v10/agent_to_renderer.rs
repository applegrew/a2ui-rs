use serde::{Deserialize, Serialize};

use crate::common::SurfaceId;

use super::common_types::{FunctionCall, FunctionResponse};

/// A single agent-to-renderer message in the v1.0 A2UI stream.
///
/// Each message must contain exactly one of the six message types, plus a
/// mandatory `version` field set to `"v1.0"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentToRendererMessage {
    /// Protocol version — always `"v1.0"`.
    pub version: String,

    /// Creates a new surface and begins rendering it.
    #[serde(rename = "createSurface", skip_serializing_if = "Option::is_none")]
    pub create_surface: Option<CreateSurface>,

    /// Updates a surface with new or modified components.
    #[serde(rename = "updateComponents", skip_serializing_if = "Option::is_none")]
    pub update_components: Option<UpdateComponents>,

    /// Updates the data model for a surface.
    #[serde(rename = "updateDataModel", skip_serializing_if = "Option::is_none")]
    pub update_data_model: Option<UpdateDataModel>,

    /// Deletes a surface.
    #[serde(rename = "deleteSurface", skip_serializing_if = "Option::is_none")]
    pub delete_surface: Option<DeleteSurface>,

    /// Executes a function locally on the renderer on behalf of the agent.
    #[serde(rename = "callRendererFunction", skip_serializing_if = "Option::is_none")]
    pub call_renderer_function: Option<CallRendererFunction>,

    /// Returns the result of a renderer-initiated `callAgentFunction` request.
    #[serde(rename = "agentFunctionResponse", skip_serializing_if = "Option::is_none")]
    pub agent_function_response: Option<FunctionResponse>,
}

/// Signals the renderer to create a new surface and begin rendering it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSurface {
    /// The unique identifier for the UI surface to be rendered.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,

    /// Default catalog for this surface (optional in v1.0; components and function
    /// calls may specify their own `catalogId`).
    #[serde(rename = "catalogId", skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,

    /// If true, the renderer sends the full data model in every message metadata.
    #[serde(rename = "sendDataModel", skip_serializing_if = "Option::is_none")]
    pub send_data_model: Option<bool>,

    /// Initial component tree, embedded directly in the surface creation payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,

    /// Initial root data model object for the surface.
    #[serde(rename = "dataModel", skip_serializing_if = "Option::is_none")]
    pub data_model: Option<serde_json::Value>,

    /// Optional surface-level metadata (vendor extensions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Updates a surface with a new set of components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateComponents {
    /// The unique identifier for the UI surface to be updated.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,

    /// A list of component objects (flat adjacency list).
    /// Component properties are catalog-defined, so they are represented as generic JSON.
    pub components: Vec<serde_json::Value>,
}

/// Updates the data model for a surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDataModel {
    /// The unique identifier for the UI surface.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,

    /// A JSON Pointer to the location in the data model to update.
    /// Defaults to `/` (entire data model) if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The new value for the specified path. Set to `null` to delete the key.
    pub value: serde_json::Value,
}

/// Signals the renderer to delete a surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteSurface {
    /// The unique identifier for the UI surface to be deleted.
    #[serde(rename = "surfaceId")]
    pub surface_id: SurfaceId,
}

/// Executes a function registered on the renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallRendererFunction {
    /// A unique identifier for this invocation instance.
    #[serde(rename = "functionCallId")]
    pub function_call_id: String,

    /// The description of the function call.
    #[serde(rename = "callFunction")]
    pub call_function: FunctionCall,
}
