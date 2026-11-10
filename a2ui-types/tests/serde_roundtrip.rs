use serde_json::json;

// ---------------------------------------------------------------------------
// v0.9 round-trip tests
// ---------------------------------------------------------------------------

mod v09 {
    use super::*;
    use a2ui_types::v09::capabilities::*;
    use a2ui_types::v09::catalog::*;
    use a2ui_types::v09::client_data_model::*;
    use a2ui_types::v09::client_to_server::*;
    use a2ui_types::v09::common_types::*;
    use a2ui_types::v09::server_to_client::*;

    #[test]
    fn roundtrip_create_surface() {
        let raw = json!({
            "version": "v0.9",
            "createSurface": {
                "surfaceId": "user_profile_card",
                "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json",
                "theme": { "primaryColor": "#00BFFF" },
                "sendDataModel": true
            }
        });

        let msg: ServerToClientMessage = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(msg.version, "v0.9");
        let cs = msg.create_surface.as_ref().unwrap();
        assert_eq!(cs.surface_id.as_str(), "user_profile_card");
        assert_eq!(
            cs.catalog_id,
            "https://a2ui.org/specification/v0_9/basic_catalog.json"
        );
        assert_eq!(cs.send_data_model, Some(true));

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_update_components() {
        let raw = json!({
            "version": "v0.9",
            "updateComponents": {
                "surfaceId": "user_profile_card",
                "components": [
                    {
                        "id": "root",
                        "component": "Column",
                        "children": ["user_name", "user_title"]
                    },
                    {
                        "id": "user_name",
                        "component": "Text",
                        "text": "John Doe"
                    }
                ]
            }
        });

        let msg: ServerToClientMessage = serde_json::from_value(raw.clone()).unwrap();
        let uc = msg.update_components.as_ref().unwrap();
        assert_eq!(uc.surface_id.as_str(), "user_profile_card");
        assert_eq!(uc.components.len(), 2);

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_update_data_model() {
        let raw = json!({
            "version": "v0.9",
            "updateDataModel": {
                "surfaceId": "user_profile_card",
                "path": "/user/name",
                "value": "Jane Doe"
            }
        });

        let msg: ServerToClientMessage = serde_json::from_value(raw.clone()).unwrap();
        let udm = msg.update_data_model.as_ref().unwrap();
        assert_eq!(udm.surface_id.as_str(), "user_profile_card");
        assert_eq!(udm.path.as_ref().unwrap(), "/user/name");
        assert_eq!(udm.value.as_ref().unwrap(), "Jane Doe");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_delete_surface() {
        let raw = json!({
            "version": "v0.9",
            "deleteSurface": {
                "surfaceId": "user_profile_card"
            }
        });

        let msg: ServerToClientMessage = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(
            msg.delete_surface.as_ref().unwrap().surface_id.as_str(),
            "user_profile_card"
        );

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_action_message() {
        let raw = json!({
            "version": "v0.9",
            "action": {
                "name": "submit_reservation",
                "surfaceId": "booking-surface",
                "sourceComponentId": "submit-btn",
                "timestamp": "2026-02-25T10:40:00Z",
                "context": {
                    "time": "7:00 PM",
                    "size": 4
                }
            }
        });

        let msg: ClientToServerMessage = serde_json::from_value(raw.clone()).unwrap();
        let action = msg.action.as_ref().unwrap();
        assert_eq!(action.name, "submit_reservation");
        assert_eq!(action.surface_id.as_str(), "booking-surface");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_error_message() {
        let raw = json!({
            "version": "v0.9",
            "error": {
                "code": "VALIDATION_FAILED",
                "surfaceId": "flight-status-card-123",
                "path": "/components/FlightCard/flightNumber",
                "message": "Missing required property 'flightNumber' in component 'FlightCard'."
            }
        });

        let msg: ClientToServerMessage = serde_json::from_value(raw.clone()).unwrap();
        let err = msg.error.as_ref().unwrap();
        assert_eq!(err.code, "VALIDATION_FAILED");
        assert_eq!(err.surface_id.as_str(), "flight-status-card-123");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_client_capabilities() {
        let raw = json!({
            "v0.9": {
                "supportedCatalogIds": [
                    "https://a2ui.org/specification/v0_9/basic_catalog.json",
                    "https://my-company.com/catalogs/v1/custom.json"
                ]
            }
        });

        let caps: ClientCapabilities = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(caps.v09.supported_catalog_ids.len(), 2);

        let reserialized = serde_json::to_value(&caps).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_server_capabilities() {
        let raw = json!({
            "v0.9": {
                "supportedCatalogIds": [
                    "https://a2ui.org/specification/v0_9/basic_catalog.json"
                ],
                "acceptsInlineCatalogs": true
            }
        });

        let caps: ServerCapabilities = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(caps.v09.accepts_inline_catalogs, Some(true));

        let reserialized = serde_json::to_value(&caps).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_client_data_model() {
        let raw = json!({
            "version": "v0.9",
            "surfaces": {
                "booking-surface": {
                    "reservationTime": "7:00 PM",
                    "partySize": 4
                }
            }
        });

        let cdm: ClientDataModel = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(cdm.version, "v0.9");
        assert!(cdm.surfaces.contains_key("booking-surface"));

        let reserialized = serde_json::to_value(&cdm).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_catalog() {
        let raw = json!({
            "catalogId": "https://example.com/catalog.json",
            "components": {
                "Text": { "type": "object", "properties": { "text": { "type": "string" } } }
            },
            "functions": [
                {
                    "name": "required",
                    "parameters": { "type": "object" },
                    "returnType": "boolean"
                }
            ]
        });

        let catalog: Catalog = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(catalog.catalog_id, "https://example.com/catalog.json");
        assert!(catalog.components.is_some());
        assert_eq!(catalog.functions.as_ref().unwrap().len(), 1);
        assert_eq!(catalog.functions.as_ref().unwrap()[0].name, "required");

        let reserialized = serde_json::to_value(&catalog).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_dynamic_string_literal() {
        let raw = json!("Hello, world!");
        let ds: DynamicString = serde_json::from_value(raw.clone()).unwrap();
        assert!(matches!(ds, DynamicString::Literal(ref s) if s == "Hello, world!"));
        let reserialized = serde_json::to_value(&ds).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_dynamic_string_binding() {
        let raw = json!({ "path": "/user/name" });
        let ds: DynamicString = serde_json::from_value(raw.clone()).unwrap();
        assert!(matches!(ds, DynamicString::Binding(ref b) if b.path == "/user/name"));
        let reserialized = serde_json::to_value(&ds).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_child_list_static() {
        let raw = json!(["child1", "child2", "child3"]);
        let cl: ChildList = serde_json::from_value(raw.clone()).unwrap();
        assert!(matches!(cl, ChildList::Static(ref v) if v.len() == 3));
        let reserialized = serde_json::to_value(&cl).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_child_list_template() {
        let raw = json!({
            "componentId": "employee_card_template",
            "path": "/employees"
        });
        let cl: ChildList = serde_json::from_value(raw.clone()).unwrap();
        assert!(matches!(cl, ChildList::Template(ref t) if t.path == "/employees"));
        let reserialized = serde_json::to_value(&cl).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_action_server_event() {
        let raw = json!({
            "event": {
                "name": "submit_form",
                "context": { "itemId": "123" }
            }
        });
        let action: Action = serde_json::from_value(raw.clone()).unwrap();
        assert!(action.event.is_some());
        assert_eq!(action.event.as_ref().unwrap().name, "submit_form");
        let reserialized = serde_json::to_value(&action).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_action_local_function() {
        let raw = json!({
            "functionCall": {
                "call": "openUrl",
                "args": { "url": "https://example.com" }
            }
        });
        let action: Action = serde_json::from_value(raw.clone()).unwrap();
        assert!(action.function_call.is_some());
        assert_eq!(action.function_call.as_ref().unwrap().call, "openUrl");
        let reserialized = serde_json::to_value(&action).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_check_rule() {
        let raw = json!({
            "condition": { "path": "/formData/email" },
            "message": "Email is required."
        });
        let rule: CheckRule = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(rule.message, "Email is required.");
        let reserialized = serde_json::to_value(&rule).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_function_call() {
        let raw = json!({
            "call": "formatString",
            "args": { "value": "Hello, ${/user/firstName}!" },
            "returnType": "string"
        });
        let fc: FunctionCall = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(fc.call, "formatString");
        assert_eq!(
            fc.return_type.as_ref().unwrap(),
            &FunctionReturnType::String
        );
        let reserialized = serde_json::to_value(&fc).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_create_surface_v091() {
        let raw = json!({
            "version": "v0.9.1",
            "createSurface": {
                "surfaceId": "user_profile_card",
                "catalogId": "https://a2ui.org/specification/v0_9_1/catalogs/basic/catalog.json",
                "sendDataModel": true
            }
        });

        let msg: ServerToClientMessage = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(msg.version, "v0.9.1");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_action_message_v091() {
        let raw = json!({
            "version": "v0.9.1",
            "action": {
                "name": "submit_reservation",
                "surfaceId": "booking-surface",
                "sourceComponentId": "submit-btn",
                "timestamp": "2026-02-25T10:40:00Z",
                "context": {}
            }
        });

        let msg: ClientToServerMessage = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(msg.version, "v0.9.1");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }
}

// ---------------------------------------------------------------------------
// v1.0 round-trip tests
// ---------------------------------------------------------------------------

mod v10 {
    use super::*;
    use a2ui_types::v10::agent_to_renderer::*;
    use a2ui_types::v10::capabilities::*;
    use a2ui_types::v10::catalog::*;
    use a2ui_types::v10::common_types::*;
    use a2ui_types::v10::renderer_to_agent::*;

    #[test]
    fn roundtrip_create_surface_with_embedded_tree() {
        let raw = json!({
            "version": "v1.0",
            "createSurface": {
                "surfaceId": "user_profile_card",
                "catalogId": "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json",
                "components": [
                    { "id": "root", "component": "Column", "children": ["user_name"] },
                    { "id": "user_name", "component": "Text", "text": { "@path": "/name" } }
                ],
                "dataModel": { "name": "John Doe" }
            }
        });

        let msg: AgentToRendererMessage = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(msg.version, "v1.0");
        let cs = msg.create_surface.as_ref().unwrap();
        assert_eq!(cs.surface_id.as_str(), "user_profile_card");
        assert_eq!(cs.components.as_ref().unwrap().len(), 2);
        assert!(cs.data_model.is_some());

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_update_data_model_v10() {
        let raw = json!({
            "version": "v1.0",
            "updateDataModel": {
                "surfaceId": "user_profile_card",
                "path": "/user/name",
                "value": "Jane Doe"
            }
        });

        let msg: AgentToRendererMessage = serde_json::from_value(raw.clone()).unwrap();
        let udm = msg.update_data_model.as_ref().unwrap();
        assert_eq!(udm.path.as_ref().unwrap(), "/user/name");
        assert_eq!(udm.value, json!("Jane Doe"));

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_call_renderer_function() {
        let raw = json!({
            "version": "v1.0",
            "callRendererFunction": {
                "functionCallId": "get_device_resolution_123",
                "callFunction": {
                    "@call": "getScreenResolution",
                    "catalogId": "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json",
                    "args": { "screenIndex": 0 }
                }
            }
        });

        let msg: AgentToRendererMessage = serde_json::from_value(raw.clone()).unwrap();
        let crf = msg.call_renderer_function.as_ref().unwrap();
        assert_eq!(crf.function_call_id, "get_device_resolution_123");
        assert_eq!(crf.call_function.call, "getScreenResolution");
        assert_eq!(crf.call_function.catalog_id.as_deref().unwrap(), "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_agent_function_response_value() {
        let raw = json!({
            "version": "v1.0",
            "agentFunctionResponse": {
                "functionCallId": "verify_provider_99",
                "value": { "valid": true }
            }
        });

        let msg: AgentToRendererMessage = serde_json::from_value(raw.clone()).unwrap();
        let resp = msg.agent_function_response.as_ref().unwrap();
        assert_eq!(resp.function_call_id, "verify_provider_99");
        assert_eq!(resp.value.as_ref().unwrap(), &json!({"valid": true}));

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_agent_function_response_error() {
        let raw = json!({
            "version": "v1.0",
            "agentFunctionResponse": {
                "functionCallId": "verify_provider_99",
                "error": { "code": "PROVIDER_NOT_FOUND", "message": "not found" }
            }
        });

        let msg: AgentToRendererMessage = serde_json::from_value(raw.clone()).unwrap();
        let resp = msg.agent_function_response.as_ref().unwrap();
        assert_eq!(resp.error.as_ref().unwrap().code, "PROVIDER_NOT_FOUND");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_action() {
        let raw = json!({
            "version": "v1.0",
            "action": {
                "name": "submit_reservation",
                "userMessage": "Submitted a reservation",
                "surfaceId": "booking-surface",
                "sourceComponentId": "submit-btn",
                "timestamp": "2026-02-25T10:40:00Z",
                "context": { "size": 4 }
            }
        });

        let msg: RendererToAgentMessage = serde_json::from_value(raw.clone()).unwrap();
        let action = msg.action.as_ref().unwrap();
        assert_eq!(action.name, "submit_reservation");
        assert_eq!(action.user_message.as_deref().unwrap(), "Submitted a reservation");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_call_agent_function() {
        let raw = json!({
            "version": "v1.0",
            "callAgentFunction": {
                "surfaceId": "s1",
                "functionCallId": "fc_1",
                "callFunction": {
                    "@call": "verifyProvider",
                    "args": { "providerId": "PRV-102" }
                }
            }
        });

        let msg: RendererToAgentMessage = serde_json::from_value(raw.clone()).unwrap();
        let caf = msg.call_agent_function.as_ref().unwrap();
        assert_eq!(caf.call_function.call, "verifyProvider");

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_function_response() {
        let raw = json!({
            "version": "v1.0",
            "rendererFunctionResponse": {
                "functionCallId": "get_device_resolution_123",
                "value": [1920, 1080]
            }
        });

        let msg: RendererToAgentMessage = serde_json::from_value(raw.clone()).unwrap();
        assert!(msg.renderer_function_response.is_some());

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_validation_error() {
        let raw = json!({
            "version": "v1.0",
            "error": {
                "code": "VALIDATION_FAILED",
                "surfaceId": "s1",
                "path": "/components/0/text",
                "message": "Missing required property"
            }
        });

        let msg: RendererToAgentMessage = serde_json::from_value(raw.clone()).unwrap();
        match msg.error.as_ref().unwrap() {
            RendererError::ValidationFailed { surface_id, .. } => {
                assert_eq!(surface_id.as_str(), "s1");
            }
            _ => panic!("expected validation failed error"),
        }

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_generic_error_with_function_call() {
        let raw = json!({
            "version": "v1.0",
            "error": {
                "code": "INVALID_FUNCTION_CALL",
                "message": "Function is rendererOnly",
                "functionCallId": "fc_1"
            }
        });

        let msg: RendererToAgentMessage = serde_json::from_value(raw.clone()).unwrap();
        match msg.error.as_ref().unwrap() {
            RendererError::Generic { function_call_id, .. } => {
                assert_eq!(function_call_id.as_deref().unwrap(), "fc_1");
            }
            _ => panic!("expected generic error"),
        }

        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_data_binding() {
        let raw = json!({ "@path": "/user/name" });
        let binding: DataBinding = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(binding.path, "/user/name");
        let reserialized = serde_json::to_value(&binding).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_function_call() {
        let raw = json!({
            "@call": "formatString",
            "args": { "value": "Hello" }
        });
        let fc: FunctionCall = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(fc.call, "formatString");
        let reserialized = serde_json::to_value(&fc).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_dynamic_string_binding() {
        let raw = json!({ "@path": "/user/name" });
        let ds: DynamicString = serde_json::from_value(raw.clone()).unwrap();
        assert!(matches!(ds, DynamicString::Binding(ref b) if b.path == "/user/name"));
        let reserialized = serde_json::to_value(&ds).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_renderer_capabilities() {
        let raw = json!({
            "v1.0": {
                "supportedCatalogIds": [
                    "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json"
                ]
            }
        });

        let caps: RendererCapabilities = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(caps.v10.supported_catalog_ids.len(), 1);

        let reserialized = serde_json::to_value(&caps).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_agent_capabilities() {
        let raw = json!({
            "v1.0": {
                "supportedCatalogIds": [
                    "https://a2ui.org/specification/v1_0/catalogs/basic/catalog.json"
                ],
                "acceptsInlineCatalogs": true
            }
        });

        let caps: AgentCapabilities = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(caps.v10.accepts_inline_catalogs, Some(true));

        let reserialized = serde_json::to_value(&caps).unwrap();
        assert_eq!(reserialized, raw);
    }

    #[test]
    fn roundtrip_catalog() {
        let raw = json!({
            "catalogId": "https://example.com/catalog.json",
            "protocolVersion": "1.0",
            "components": {
                "Text": { "type": "object" }
            }
        });

        let catalog: Catalog = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(catalog.catalog_id, "https://example.com/catalog.json");
        assert_eq!(catalog.protocol_version.as_deref().unwrap(), "1.0");
        assert!(catalog.components.is_some());

        let reserialized = serde_json::to_value(&catalog).unwrap();
        assert_eq!(reserialized, raw);
    }
}
