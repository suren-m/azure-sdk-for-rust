#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod workspaces {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<Workspace, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: Workspace = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        parameters: &Workspace,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Workspace = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Workspace = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Workspace),
            Created201(Workspace),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        parameters: &WorkspaceUpdate,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<update::Response, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: Workspace = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(update::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Workspace),
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::OK => Ok(delete::Response::Ok200),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Accepted202,
            Ok200,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<WorkspaceListResult, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: WorkspaceListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                list_by_resource_group::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn list_by_subscription(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<WorkspaceListResult, list_by_subscription::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Databricks/workspaces",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_subscription::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_subscription::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_subscription::ResponseBytesError)?;
                let rsp_value: WorkspaceListResult =
                    serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_subscription::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
                list_by_subscription::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_subscription {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
}
pub mod v_net_peering {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
        peering_name: &str,
    ) -> std::result::Result<get::Response, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}/virtualNetworkPeerings/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name, peering_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: VirtualNetworkPeering = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(get::Response::Ok200(rsp_value))
            }
            StatusCode::NO_CONTENT => Ok(get::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(VirtualNetworkPeering),
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        virtual_network_peering_parameters: &VirtualNetworkPeering,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
        peering_name: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}/virtualNetworkPeerings/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name, peering_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(virtual_network_peering_parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: VirtualNetworkPeering =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: VirtualNetworkPeering =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(VirtualNetworkPeering),
            Ok200(VirtualNetworkPeering),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
        peering_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}/virtualNetworkPeerings/{}",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name, peering_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            StatusCode::OK => Ok(delete::Response::Ok200),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
            Ok200,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn list_by_workspace(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        workspace_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<VirtualNetworkPeeringList, list_by_workspace::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Databricks/workspaces/{}/virtualNetworkPeerings",
            &operation_config.base_path, subscription_id, resource_group_name, workspace_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_workspace::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_workspace::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_workspace::ResponseBytesError)?;
                let rsp_value: VirtualNetworkPeeringList =
                    serde_json::from_slice(&body).context(list_by_workspace::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_workspace::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list_by_workspace::DeserializeError { body })?;
                list_by_workspace::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_workspace {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!("{}/providers/Microsoft.Databricks/operations", &operation_config.base_path,);
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: OperationListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
}
