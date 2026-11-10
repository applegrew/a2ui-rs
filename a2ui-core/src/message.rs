use a2ui_types::common::{SpecVersion, SurfaceId};
use a2ui_types::v09::server_to_client as v09;
use a2ui_types::v10::agent_to_renderer as v10;
use a2ui_types::v10::common_types::{FunctionCall, FunctionResponse};

// ---------------------------------------------------------------------------
// v0.9 Message Builders
// ---------------------------------------------------------------------------

/// Builder for a v0.9 `createSurface` message.
pub struct CreateSurfaceBuilder {
    surface_id: String,
    catalog_id: String,
    theme: Option<serde_json::Value>,
    send_data_model: Option<bool>,
    spec_version: SpecVersion,
}

impl CreateSurfaceBuilder {
    pub fn new(surface_id: impl Into<String>, catalog_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            catalog_id: catalog_id.into(),
            theme: None,
            send_data_model: None,
            spec_version: SpecVersion::V0_9_1,
        }
    }

    pub fn theme(mut self, theme: serde_json::Value) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn send_data_model(mut self, send: bool) -> Self {
        self.send_data_model = Some(send);
        self
    }

    /// Override the emitted protocol `version` string (defaults to `"v0.9.1"`).
    pub fn spec_version(mut self, version: SpecVersion) -> Self {
        self.spec_version = version;
        self
    }

    pub fn build(self) -> v09::ServerToClientMessage {
        v09::ServerToClientMessage {
            version: self.spec_version.to_string(),
            create_surface: Some(v09::CreateSurface {
                surface_id: SurfaceId::new(self.surface_id),
                catalog_id: self.catalog_id,
                theme: self.theme,
                send_data_model: self.send_data_model,
            }),
            update_components: None,
            update_data_model: None,
            delete_surface: None,
        }
    }
}

/// Builder for a v0.9 `updateComponents` message.
pub struct UpdateComponentsBuilder {
    surface_id: String,
    components: Vec<serde_json::Value>,
    spec_version: SpecVersion,
}

impl UpdateComponentsBuilder {
    pub fn new(surface_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            components: Vec::new(),
            spec_version: SpecVersion::V0_9_1,
        }
    }

    pub fn add_component(mut self, component: serde_json::Value) -> Self {
        self.components.push(component);
        self
    }

    pub fn components(mut self, components: Vec<serde_json::Value>) -> Self {
        self.components = components;
        self
    }

    /// Override the emitted protocol `version` string (defaults to `"v0.9.1"`).
    pub fn spec_version(mut self, version: SpecVersion) -> Self {
        self.spec_version = version;
        self
    }

    pub fn build(self) -> v09::ServerToClientMessage {
        v09::ServerToClientMessage {
            version: self.spec_version.to_string(),
            create_surface: None,
            update_components: Some(v09::UpdateComponents {
                surface_id: SurfaceId::new(self.surface_id),
                components: self.components,
            }),
            update_data_model: None,
            delete_surface: None,
        }
    }
}

/// Builder for a v0.9 `updateDataModel` message.
pub struct UpdateDataModelBuilder {
    surface_id: String,
    path: Option<String>,
    value: Option<serde_json::Value>,
    spec_version: SpecVersion,
}

impl UpdateDataModelBuilder {
    pub fn new(surface_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            path: None,
            value: None,
            spec_version: SpecVersion::V0_9_1,
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Override the emitted protocol `version` string (defaults to `"v0.9.1"`).
    pub fn spec_version(mut self, version: SpecVersion) -> Self {
        self.spec_version = version;
        self
    }

    pub fn build(self) -> v09::ServerToClientMessage {
        v09::ServerToClientMessage {
            version: self.spec_version.to_string(),
            create_surface: None,
            update_components: None,
            update_data_model: Some(v09::UpdateDataModel {
                surface_id: SurfaceId::new(self.surface_id),
                path: self.path,
                value: self.value,
            }),
            delete_surface: None,
        }
    }
}

/// Build a v0.9 `deleteSurface` message (defaults to protocol version `"v0.9.1"`).
pub fn delete_surface_v09(surface_id: impl Into<String>) -> v09::ServerToClientMessage {
    delete_surface_v09_with_version(surface_id, SpecVersion::V0_9_1)
}

/// Build a v0.9 `deleteSurface` message with an explicit protocol version.
pub fn delete_surface_v09_with_version(
    surface_id: impl Into<String>,
    spec_version: SpecVersion,
) -> v09::ServerToClientMessage {
    v09::ServerToClientMessage {
        version: spec_version.to_string(),
        create_surface: None,
        update_components: None,
        update_data_model: None,
        delete_surface: Some(v09::DeleteSurface {
            surface_id: SurfaceId::new(surface_id),
        }),
    }
}

// ---------------------------------------------------------------------------
// v1.0 Message Builders
// ---------------------------------------------------------------------------

/// Builder for a v1.0 `createSurface` message.
pub struct CreateSurfaceBuilderV10 {
    surface_id: String,
    catalog_id: Option<String>,
    send_data_model: Option<bool>,
    components: Option<Vec<serde_json::Value>>,
    data_model: Option<serde_json::Value>,
    metadata: Option<serde_json::Value>,
}

impl CreateSurfaceBuilderV10 {
    pub fn new(surface_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            catalog_id: None,
            send_data_model: None,
            components: None,
            data_model: None,
            metadata: None,
        }
    }

    /// Set the default catalog for this surface (optional in v1.0).
    pub fn catalog_id(mut self, catalog_id: impl Into<String>) -> Self {
        self.catalog_id = Some(catalog_id.into());
        self
    }

    pub fn send_data_model(mut self, send: bool) -> Self {
        self.send_data_model = Some(send);
        self
    }

    /// Provide the initial component tree embedded in the surface creation payload.
    pub fn components(mut self, components: Vec<serde_json::Value>) -> Self {
        self.components = Some(components);
        self
    }

    /// Add a single component to the initial component tree.
    pub fn add_component(mut self, component: serde_json::Value) -> Self {
        self.components
            .get_or_insert_with(Vec::new)
            .push(component);
        self
    }

    /// Provide the initial root data model object.
    pub fn data_model(mut self, data_model: serde_json::Value) -> Self {
        self.data_model = Some(data_model);
        self
    }

    pub fn build(self) -> v10::AgentToRendererMessage {
        v10::AgentToRendererMessage {
            version: "v1.0".to_string(),
            create_surface: Some(v10::CreateSurface {
                surface_id: SurfaceId::new(self.surface_id),
                catalog_id: self.catalog_id,
                send_data_model: self.send_data_model,
                components: self.components,
                data_model: self.data_model,
                metadata: self.metadata,
            }),
            update_components: None,
            update_data_model: None,
            delete_surface: None,
            call_renderer_function: None,
            agent_function_response: None,
        }
    }
}

/// Builder for a v1.0 `updateComponents` message.
pub struct UpdateComponentsBuilderV10 {
    surface_id: String,
    components: Vec<serde_json::Value>,
}

impl UpdateComponentsBuilderV10 {
    pub fn new(surface_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            components: Vec::new(),
        }
    }

    pub fn add_component(mut self, component: serde_json::Value) -> Self {
        self.components.push(component);
        self
    }

    pub fn components(mut self, components: Vec<serde_json::Value>) -> Self {
        self.components = components;
        self
    }

    pub fn build(self) -> v10::AgentToRendererMessage {
        v10::AgentToRendererMessage {
            version: "v1.0".to_string(),
            create_surface: None,
            update_components: Some(v10::UpdateComponents {
                surface_id: SurfaceId::new(self.surface_id),
                components: self.components,
            }),
            update_data_model: None,
            delete_surface: None,
            call_renderer_function: None,
            agent_function_response: None,
        }
    }
}

/// Builder for a v1.0 `updateDataModel` message.
pub struct UpdateDataModelBuilderV10 {
    surface_id: String,
    path: Option<String>,
    value: serde_json::Value,
}

impl UpdateDataModelBuilderV10 {
    /// Create a data model update. The `value` is required in v1.0.
    pub fn new(surface_id: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            surface_id: surface_id.into(),
            path: None,
            value,
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(self) -> v10::AgentToRendererMessage {
        v10::AgentToRendererMessage {
            version: "v1.0".to_string(),
            create_surface: None,
            update_components: None,
            update_data_model: Some(v10::UpdateDataModel {
                surface_id: SurfaceId::new(self.surface_id),
                path: self.path,
                value: self.value,
            }),
            delete_surface: None,
            call_renderer_function: None,
            agent_function_response: None,
        }
    }
}

/// Build a v1.0 `deleteSurface` message.
pub fn delete_surface_v10(surface_id: impl Into<String>) -> v10::AgentToRendererMessage {
    v10::AgentToRendererMessage {
        version: "v1.0".to_string(),
        create_surface: None,
        update_components: None,
        update_data_model: None,
        delete_surface: Some(v10::DeleteSurface {
            surface_id: SurfaceId::new(surface_id),
        }),
        call_renderer_function: None,
        agent_function_response: None,
    }
}

/// Build a v1.0 `callRendererFunction` message.
pub fn call_renderer_function_v10(
    function_call_id: impl Into<String>,
    call_function: FunctionCall,
) -> v10::AgentToRendererMessage {
    v10::AgentToRendererMessage {
        version: "v1.0".to_string(),
        create_surface: None,
        update_components: None,
        update_data_model: None,
        delete_surface: None,
        call_renderer_function: Some(v10::CallRendererFunction {
            function_call_id: function_call_id.into(),
            call_function,
        }),
        agent_function_response: None,
    }
}

/// Build a v1.0 `agentFunctionResponse` message from a `FunctionResponse`.
pub fn agent_function_response_v10(
    response: FunctionResponse,
) -> v10::AgentToRendererMessage {
    v10::AgentToRendererMessage {
        version: "v1.0".to_string(),
        create_surface: None,
        update_components: None,
        update_data_model: None,
        delete_surface: None,
        call_renderer_function: None,
        agent_function_response: Some(response),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_surface_builder() {
        let msg = CreateSurfaceBuilder::new("my-surface", "https://example.com/catalog.json")
            .theme(json!({"primaryColor": "#FF0000"}))
            .send_data_model(true)
            .build();

        assert_eq!(msg.version, "v0.9.1");
        let cs = msg.create_surface.unwrap();
        assert_eq!(cs.surface_id.as_str(), "my-surface");
        assert_eq!(cs.catalog_id, "https://example.com/catalog.json");
        assert!(cs.theme.is_some());
        assert_eq!(cs.send_data_model, Some(true));
    }

    #[test]
    fn test_create_surface_builder_explicit_v09() {
        let msg = CreateSurfaceBuilder::new("my-surface", "https://example.com/catalog.json")
            .spec_version(SpecVersion::V0_9)
            .build();

        assert_eq!(msg.version, "v0.9");
    }

    #[test]
    fn test_update_components_builder() {
        let msg = UpdateComponentsBuilder::new("s1")
            .add_component(json!({"id": "root", "component": "Column", "children": ["t1"]}))
            .add_component(json!({"id": "t1", "component": "Text", "text": "Hello"}))
            .build();

        let uc = msg.update_components.unwrap();
        assert_eq!(uc.components.len(), 2);
    }

    #[test]
    fn test_update_data_model_builder() {
        let msg = UpdateDataModelBuilder::new("s1")
            .path("/user/name")
            .value(json!("Alice"))
            .build();

        let udm = msg.update_data_model.unwrap();
        assert_eq!(udm.path.unwrap(), "/user/name");
        assert_eq!(udm.value.unwrap(), json!("Alice"));
    }

    #[test]
    fn test_delete_surface_v09() {
        let msg = delete_surface_v09("s1");
        let ds = msg.delete_surface.unwrap();
        assert_eq!(ds.surface_id.as_str(), "s1");
    }

    #[test]
    fn test_create_surface_builder_v10() {
        let msg = CreateSurfaceBuilderV10::new("s1")
            .catalog_id("https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json")
            .send_data_model(true)
            .add_component(json!({"id": "root", "component": "Column"}))
            .data_model(json!({"name": "Alice"}))
            .build();

        assert_eq!(msg.version, "v1.0");
        let cs = msg.create_surface.unwrap();
        assert_eq!(cs.surface_id.as_str(), "s1");
        assert!(cs.catalog_id.is_some());
        assert_eq!(cs.components.as_ref().unwrap().len(), 1);
        assert!(cs.data_model.is_some());
    }

    #[test]
    fn test_update_data_model_builder_v10() {
        let msg = UpdateDataModelBuilderV10::new("s1", json!("Alice"))
            .path("/user/name")
            .build();

        let udm = msg.update_data_model.unwrap();
        assert_eq!(udm.path.unwrap(), "/user/name");
        assert_eq!(udm.value, json!("Alice"));
    }

    #[test]
    fn test_delete_surface_v10() {
        let msg = delete_surface_v10("s1");
        let ds = msg.delete_surface.unwrap();
        assert_eq!(ds.surface_id.as_str(), "s1");
    }

    #[test]
    fn test_call_renderer_function_v10() {
        let call = FunctionCall {
            call: "getScreenResolution".to_string(),
            catalog_id: Some("https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json".to_string()),
            args: Some(json!({"screenIndex": 0})),
        };
        let msg = call_renderer_function_v10("call_1", call);

        let crf = msg.call_renderer_function.unwrap();
        assert_eq!(crf.function_call_id, "call_1");
        assert_eq!(crf.call_function.call, "getScreenResolution");
    }

    #[test]
    fn test_agent_function_response_v10() {
        let msg = agent_function_response_v10(FunctionResponse {
            function_call_id: "call_1".to_string(),
            value: Some(json!([1920, 1080])),
            error: None,
        });

        let resp = msg.agent_function_response.unwrap();
        assert_eq!(resp.function_call_id, "call_1");
        assert_eq!(resp.value.unwrap(), json!([1920, 1080]));
    }
}
