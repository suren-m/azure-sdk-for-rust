#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod configuration_stores {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        skip_token: Option<&str>,
    ) -> std::result::Result<ConfigurationStoreListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AppConfiguration/configurationStores",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ConfigurationStoreListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        skip_token: Option<&str>,
    ) -> std::result::Result<ConfigurationStoreListResult, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: ConfigurationStoreListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
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
        config_store_name: &str,
    ) -> std::result::Result<ConfigurationStore, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
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
                let rsp_value: ConfigurationStore = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        config_store_name: &str,
        config_store_creation_parameters: &ConfigurationStore,
    ) -> std::result::Result<create::Response, create::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(config_store_creation_parameters);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: ConfigurationStore = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: ConfigurationStore = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                create::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(ConfigurationStore),
            Created201(ConfigurationStore),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
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
        config_store_name: &str,
        config_store_update_parameters: &ConfigurationStoreUpdateParameters,
    ) -> std::result::Result<update::Response, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(config_store_update_parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: ConfigurationStore = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: ConfigurationStore = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
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
            Ok200(ConfigurationStore),
            Created201(ConfigurationStore),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
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
        config_store_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
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
                let rsp_value: Error = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
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
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_keys(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        config_store_name: &str,
        skip_token: Option<&str>,
    ) -> std::result::Result<ApiKeyListResult, list_keys::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}/ListKeys",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list_keys::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_keys::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                let rsp_value: ApiKeyListResult = serde_json::from_slice(&body).context(list_keys::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_keys::DeserializeError { body })?;
                list_keys::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_keys {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn regenerate_key(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        config_store_name: &str,
        regenerate_key_parameters: &RegenerateKeyParameters,
    ) -> std::result::Result<ApiKey, regenerate_key::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}/RegenerateKey",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(regenerate_key_parameters);
        let req = req_builder.build().context(regenerate_key::BuildRequestError)?;
        let rsp = client.execute(req).await.context(regenerate_key::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_key::ResponseBytesError)?;
                let rsp_value: ApiKey = serde_json::from_slice(&body).context(regenerate_key::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_key::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(regenerate_key::DeserializeError { body })?;
                regenerate_key::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod regenerate_key {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_key_value(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        config_store_name: &str,
        list_key_value_parameters: &ListKeyValueParameters,
    ) -> std::result::Result<KeyValue, list_key_value::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AppConfiguration/configurationStores/{}/listKeyValue",
            &operation_config.base_path, subscription_id, resource_group_name, config_store_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(list_key_value_parameters);
        let req = req_builder.build().context(list_key_value::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_key_value::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_key_value::ResponseBytesError)?;
                let rsp_value: KeyValue = serde_json::from_slice(&body).context(list_key_value::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_key_value::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_key_value::DeserializeError { body })?;
                list_key_value::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_key_value {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn check_name_availability(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        check_name_availability_parameters: &CheckNameAvailabilityParameters,
    ) -> std::result::Result<NameAvailabilityStatus, check_name_availability::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AppConfiguration/checkNameAvailability",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(check_name_availability_parameters);
        let req = req_builder.build().context(check_name_availability::BuildRequestError)?;
        let rsp = client.execute(req).await.context(check_name_availability::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                let rsp_value: NameAvailabilityStatus =
                    serde_json::from_slice(&body).context(check_name_availability::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(check_name_availability::DeserializeError { body })?;
                check_name_availability::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod check_name_availability {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        skip_token: Option<&str>,
    ) -> std::result::Result<OperationDefinitionListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!("{}/providers/Microsoft.AppConfiguration/operations", &operation_config.base_path,);
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: OperationDefinitionListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
