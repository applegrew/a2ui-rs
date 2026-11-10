use serde::{Deserialize, Serialize};

use crate::common::ComponentId;

// ---------------------------------------------------------------------------
// Data binding & function call — the core of v1.0 dynamic values
// ---------------------------------------------------------------------------

/// A data binding reference to the data model via JSON Pointer.
///
/// Serialized as `{"@path": "..."}`. The `@` prefix marks the key as part of the
/// protocol's reserved system namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataBinding {
    /// A JSON Pointer path to a value in the data model.
    #[serde(rename = "@path")]
    pub path: String,
}

/// Invokes a named function on the client.
///
/// Serialized as `{"@call": "...", "catalogId": "...", "args": {...}}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionCall {
    /// The name of the function to call.
    #[serde(rename = "@call")]
    pub call: String,

    /// The catalog ID defining the function, overriding any surface default.
    #[serde(rename = "catalogId", skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,

    /// Arguments passed to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Dynamic value types
// ---------------------------------------------------------------------------

/// A string that can be a literal, a data binding, or a function call returning a string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicString {
    Literal(String),
    Binding(DataBinding),
    Function(FunctionCall),
}

/// A number that can be literal, data-bound, or a function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicNumber {
    Literal(f64),
    Binding(DataBinding),
    Function(FunctionCall),
}

/// A boolean that can be literal, data-bound, or a function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicBoolean {
    Literal(bool),
    Binding(DataBinding),
    Function(FunctionCall),
}

/// A string list that can be literal, data-bound, or a function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicStringList {
    Literal(Vec<String>),
    Binding(DataBinding),
    Function(FunctionCall),
}

/// A value that can be any literal, a data binding, or a function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<serde_json::Value>),
    Binding(DataBinding),
    Function(FunctionCall),
    Object(serde_json::Value),
}

// ---------------------------------------------------------------------------
// Child list — how containers hold children
// ---------------------------------------------------------------------------

/// A template for generating children from a data model list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildListTemplate {
    /// The component to use as a template for each item.
    #[serde(rename = "componentId")]
    pub component_id: ComponentId,

    /// The path to the list in the data model.
    pub path: String,
}

/// Defines how a container holds its children.
/// Either a static array of component IDs, or a template for dynamic lists.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChildList {
    Static(Vec<ComponentId>),
    Template(ChildListTemplate),
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/// An event to dispatch to the agent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionEvent {
    /// The name of the action to dispatch.
    pub name: String,

    /// Optional human-readable description of the action performed.
    #[serde(rename = "userMessage", skip_serializing_if = "Option::is_none")]
    pub user_message: Option<DynamicString>,

    /// Key-value pairs for the action context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

/// Defines an interaction handler: either an agent-side event or a local function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Event {
        #[serde(rename = "event")]
        event: ActionEvent,
    },
    FunctionCall {
        #[serde(rename = "functionCall")]
        function_call: FunctionCall,
    },
}

// ---------------------------------------------------------------------------
// Checks / Validation
// ---------------------------------------------------------------------------

/// The condition of a validation check: a data binding or a function call
/// that evaluates to a structured validation result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCondition {
    Binding(DataBinding),
    Function(FunctionCall),
}

/// A single validation rule applied to an input component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckRule {
    /// The condition to evaluate.
    pub condition: CheckCondition,

    /// Optional fallback error message to display if the check fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// A structured validation result returned by a check condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the check passed.
    pub valid: bool,

    /// Machine-readable error code (e.g. `EXPIRED_CARD`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Human-readable error or warning message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Severity level of the validation result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ValidationSeverity>,
}

/// Severity level of a validation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

/// Attributes for assistive technologies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityAttributes {
    /// A short label for the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<DynamicString>,

    /// Additional description for the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DynamicString>,

    /// Controls screen-reader announcements for dynamic updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<AccessibilityLive>,

    /// Hides the element from assistive technologies when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<DynamicBoolean>,
}

/// Screen-reader announcement mode for dynamic updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessibilityLive {
    Off,
    Polite,
    Assertive,
}

// ---------------------------------------------------------------------------
// Function responses
// ---------------------------------------------------------------------------

/// An error reported when a function call fails.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionError {
    /// Machine-readable error code.
    pub code: String,

    /// Human-readable description of the failure.
    pub message: String,
}

/// The return response matching a `callAgentFunction` or `callRendererFunction` invocation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionResponse {
    /// The unique ID matching the initiating function call.
    #[serde(rename = "functionCallId")]
    pub function_call_id: String,

    /// The returned execution value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,

    /// An error object indicating execution failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FunctionError>,
}
