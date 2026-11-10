use serde::{Deserialize, Serialize};

/// A collection of component and function definitions that make up a catalog.
///
/// This is a pragmatic typed view of the v1.0 catalog definition. The full
/// `components` and `functions` maps are carried as raw JSON so they can be fed
/// directly into JSON Schema validation and LLM prompt generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    /// Unique identifier for this catalog.
    #[serde(rename = "catalogId")]
    pub catalog_id: String,

    /// Markdown-formatted design guidelines or instructions specific to this catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Definitions for UI components supported by this catalog (map of type name to schema).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,

    /// Definitions for functions supported by this catalog (map of name to schema).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<serde_json::Value>,

    /// The A2UI specification version of this catalog (e.g. `"1.0"`).
    #[serde(rename = "protocolVersion", skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
}
