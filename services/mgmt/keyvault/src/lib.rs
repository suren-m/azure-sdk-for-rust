#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-preview-2020-04")]
mod package_preview_2020_04;
#[cfg(feature = "package-preview-2020-04")]
pub use package_preview_2020_04::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-04-full")]
mod package_preview_2020_04_full;
#[cfg(feature = "package-preview-2020-04-full")]
pub use package_preview_2020_04_full::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-09")]
mod package_2019_09;
#[cfg(feature = "package-2019-09")]
pub use package_2019_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-02-14-preview")]
mod package_2018_02_14_preview;
#[cfg(feature = "package-2018-02-14-preview")]
pub use package_2018_02_14_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-02")]
mod package_2018_02;
#[cfg(feature = "package-2018-02")]
pub use package_2018_02::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-10")]
mod package_2016_10;
#[cfg(feature = "package-2016-10")]
pub use package_2016_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-06")]
mod package_2015_06;
#[cfg(feature = "package-2015-06")]
pub use package_2015_06::{models, operations, API_VERSION};
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
