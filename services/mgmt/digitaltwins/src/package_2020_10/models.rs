#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsProperties {
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "lastUpdatedTime", skip_serializing)]
    pub last_updated_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<digital_twins_properties::ProvisioningState>,
    #[serde(rename = "hostName", skip_serializing)]
    pub host_name: Option<String>,
}
mod digital_twins_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        Deleted,
        Warning,
        Suspending,
        Restoring,
        Moving,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsDescription {
    #[serde(flatten)]
    pub digital_twins_resource: DigitalTwinsResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DigitalTwinsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsPatchDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DigitalTwinsDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing)]
    pub is_data_action: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_request::Type,
}
mod check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DigitalTwins/digitalTwinsInstances")]
        Microsoft_DigitalTwinsDigitalTwinsInstances,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameResult {
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
mod check_name_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResource {
    #[serde(flatten)]
    pub external_resource: ExternalResource,
    pub properties: DigitalTwinsEndpointResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResourceProperties {
    #[serde(rename = "endpointType")]
    pub endpoint_type: digital_twins_endpoint_resource_properties::EndpointType,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<digital_twins_endpoint_resource_properties::ProvisioningState>,
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "deadLetterSecret", skip_serializing_if = "Option::is_none")]
    pub dead_letter_secret: Option<String>,
}
mod digital_twins_endpoint_resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        EventHub,
        EventGrid,
        ServiceBus,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        Deleted,
        Warning,
        Suspending,
        Restoring,
        Moving,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBus {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHub {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGrid {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResourceListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DigitalTwinsEndpointResource>,
}
