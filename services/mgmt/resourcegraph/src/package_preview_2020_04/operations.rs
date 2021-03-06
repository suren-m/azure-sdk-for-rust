#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub async fn resources(
    operation_config: &crate::OperationConfig,
    query: &QueryRequest,
) -> std::result::Result<QueryResponse, resources::Error> {
    let client = &operation_config.client;
    let uri_str = &format!("{}/providers/Microsoft.ResourceGraph/resources", &operation_config.base_path,);
    let mut req_builder = client.post(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(query);
    let req = req_builder.build().context(resources::BuildRequestError)?;
    let rsp = client.execute(req).await.context(resources::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(resources::ResponseBytesError)?;
            let rsp_value: QueryResponse = serde_json::from_slice(&body).context(resources::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(resources::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(resources::DeserializeError { body })?;
            resources::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod resources {
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
pub async fn resource_changes(
    operation_config: &crate::OperationConfig,
    parameters: &ResourceChangesRequestParameters,
) -> std::result::Result<ResourceChangeList, resource_changes::Error> {
    let client = &operation_config.client;
    let uri_str = &format!("{}/providers/Microsoft.ResourceGraph/resourceChanges", &operation_config.base_path,);
    let mut req_builder = client.post(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(resource_changes::BuildRequestError)?;
    let rsp = client.execute(req).await.context(resource_changes::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(resource_changes::ResponseBytesError)?;
            let rsp_value: ResourceChangeList = serde_json::from_slice(&body).context(resource_changes::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(resource_changes::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(resource_changes::DeserializeError { body })?;
            resource_changes::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod resource_changes {
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
pub async fn resource_change_details(
    operation_config: &crate::OperationConfig,
    parameters: &ResourceChangeDetailsRequestParameters,
) -> std::result::Result<ResourceChangeData, resource_change_details::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.ResourceGraph/resourceChangeDetails",
        &operation_config.base_path,
    );
    let mut req_builder = client.post(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(resource_change_details::BuildRequestError)?;
    let rsp = client.execute(req).await.context(resource_change_details::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(resource_change_details::ResponseBytesError)?;
            let rsp_value: ResourceChangeData =
                serde_json::from_slice(&body).context(resource_change_details::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(resource_change_details::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(resource_change_details::DeserializeError { body })?;
            resource_change_details::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod resource_change_details {
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
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!("{}/providers/Microsoft.ResourceGraph/operations", &operation_config.base_path,);
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
                list::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
