#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-03-preview")]
mod package_2020_03_preview;
#[cfg(feature = "package-2020-03-preview")]
pub use package_2020_03_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-pure-2020-03-preview")]
mod package_pure_2020_03_preview;
#[cfg(feature = "package-pure-2020-03-preview")]
pub use package_pure_2020_03_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-pure-2017-04-preview")]
mod package_pure_2017_04_preview;
#[cfg(feature = "package-pure-2017-04-preview")]
pub use package_pure_2017_04_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-pure-2016-03")]
mod package_pure_2016_03;
#[cfg(feature = "package-pure-2016-03")]
pub use package_pure_2016_03::{models, operations, API_VERSION};
pub struct OperationConfig {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}
impl OperationConfig {
    pub fn new(bearer_access_token: &str) -> Self {
        Self {
            bearer_access_token: Some(bearer_access_token.to_owned()),
            ..Default::default()
        }
    }
}
impl Default for OperationConfig {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            bearer_access_token: None,
        }
    }
}
