#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalSupportPlanProperties {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<canonical_support_plan_properties::ProvisioningState>,
}
mod canonical_support_plan_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Purchasing,
        Downgrading,
        Cancelling,
        Upgrading,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalSupportPlanResponseEnvelope {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub properties: CanonicalSupportPlanProperties,
}
pub type CanonicalSupportPlanStatus = Vec<serde_json::Value>;
pub type OperationList = Vec<OperationsDefinition>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationsDisplayDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDisplayDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub message: String,
    pub code: i64,
}
