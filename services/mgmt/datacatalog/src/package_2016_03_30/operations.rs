#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod adc_operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationEntityListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!("{}/providers/Microsoft.DataCatalog/operations", &operation_config.base_path,);
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
                let rsp_value: OperationEntityListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
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
pub mod adc_catalogs {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn listt_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<AdcCatalogsListResult, listt_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataCatalog/catalogs",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(listt_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(listt_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(listt_by_resource_group::ResponseBytesError)?;
                let rsp_value: AdcCatalogsListResult =
                    serde_json::from_slice(&body).context(listt_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(listt_by_resource_group::ResponseBytesError)?;
                listt_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod listt_by_resource_group {
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
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        catalog_name: &str,
    ) -> std::result::Result<AdcCatalog, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataCatalog/catalogs/{}",
            &operation_config.base_path, subscription_id, resource_group_name, catalog_name
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
                let rsp_value: AdcCatalog = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        catalog_name: &str,
        properties: &AdcCatalog,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataCatalog/catalogs/{}",
            &operation_config.base_path, subscription_id, resource_group_name, catalog_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(properties);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: AdcCatalog = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: AdcCatalog = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                create_or_update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(AdcCatalog),
            Created201(AdcCatalog),
        }
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
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        catalog_name: &str,
        properties: &AdcCatalog,
    ) -> std::result::Result<AdcCatalog, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataCatalog/catalogs/{}",
            &operation_config.base_path, subscription_id, resource_group_name, catalog_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(properties);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: AdcCatalog = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
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
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        catalog_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataCatalog/catalogs/{}",
            &operation_config.base_path, subscription_id, resource_group_name, catalog_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
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
