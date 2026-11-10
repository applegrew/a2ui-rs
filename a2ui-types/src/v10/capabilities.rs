use serde::{Deserialize, Serialize};

use super::catalog::Catalog;

// ---------------------------------------------------------------------------
// Renderer Capabilities
// ---------------------------------------------------------------------------

/// Renderer capabilities for v1.0, sent in transport metadata under
/// `a2uiRendererCapabilities`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RendererCapabilities {
    /// The capabilities structure for v1.0.
    #[serde(rename = "v1.0")]
    pub v10: RendererCapabilitiesV10,
}

/// The inner capabilities object for v1.0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RendererCapabilitiesV10 {
    /// Catalog IDs supported by the renderer. Multiple catalogs can be mixed in a surface.
    #[serde(rename = "supportedCatalogIds")]
    pub supported_catalog_ids: Vec<String>,

    /// Inline catalog definitions (only if the agent declares `acceptsInlineCatalogs: true`).
    #[serde(rename = "inlineCatalogs", skip_serializing_if = "Option::is_none")]
    pub inline_catalogs: Option<Vec<Catalog>>,
}

// ---------------------------------------------------------------------------
// Agent Capabilities
// ---------------------------------------------------------------------------

/// Agent capabilities for v1.0, advertised via Agent Card or transport initialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCapabilities {
    /// The capabilities structure for v1.0.
    #[serde(rename = "v1.0")]
    pub v10: AgentCapabilitiesV10,
}

/// The inner agent capabilities object for v1.0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCapabilitiesV10 {
    /// Catalog IDs that the agent can generate UI for.
    #[serde(rename = "supportedCatalogIds")]
    pub supported_catalog_ids: Vec<String>,

    /// Whether the agent accepts inline catalogs from the renderer.
    #[serde(rename = "acceptsInlineCatalogs", skip_serializing_if = "Option::is_none")]
    pub accepts_inline_catalogs: Option<bool>,
}
