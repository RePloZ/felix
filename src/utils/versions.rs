use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::sync::Mutex;

use crate::utils::errors::ApiErrors;

pub static API_CAPABILITIES: Lazy<Mutex<HashMap<u16, (String, RangeInclusive<u16>)>>> =
    Lazy::new(|| {
        let mut versions = HashMap::new();
        versions.insert(18, (String::from("ApiVersions"), 0..=4));
        versions.insert(75, (String::from("DescribeTopicPartitions"), 0..=0));

        Mutex::new(versions)
    });

#[derive(Debug, Clone)]
pub struct ApiReqVersion {
    api_key: u16,
    version: u16,
}

impl ApiReqVersion {
    pub fn new(api_key: u16, version: u16) -> Self {
        Self { api_key, version }
    }

    pub fn check(&self) -> Result<(), ApiErrors> {
        let api_versions = API_CAPABILITIES.lock().unwrap();
        let (_, version_range) = api_versions
            .get(&self.api_key)
            .ok_or(ApiErrors::UnsupportedKey(self.api_key))?;

        if version_range.contains(&self.version) {
            Ok(())
        } else {
            Err(ApiErrors::UnsupportedVersion(self.api_key, self.version))
        }
    }
}
