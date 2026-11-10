use serde::{Deserialize, Serialize};

/// Unique identifier for a UI surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SurfaceId(pub String);

impl SurfaceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SurfaceId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for SurfaceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Unique identifier for a component within a surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ComponentId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ComponentId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Unique identifier for a component catalog.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CatalogId(pub String);

impl CatalogId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CatalogId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for CatalogId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// The A2UI spec version this library supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecVersion {
    V0_9,
    V0_9_1,
    V1_0,
}

impl SpecVersion {
    /// Parse a protocol version string (e.g. `"v0.9"`, `"v0.9.1"`, or `"v1.0"`).
    ///
    /// Returns `None` for unknown versions.
    pub fn parse(version: &str) -> Option<Self> {
        match version {
            "v0.9" => Some(Self::V0_9),
            "v0.9.1" => Some(Self::V0_9_1),
            "v1.0" => Some(Self::V1_0),
            _ => None,
        }
    }
}

impl std::fmt::Display for SpecVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecVersion::V0_9 => write!(f, "v0.9"),
            SpecVersion::V0_9_1 => write!(f, "v0.9.1"),
            SpecVersion::V1_0 => write!(f, "v1.0"),
        }
    }
}
