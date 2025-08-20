use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: u32,
    pub local_deployment_image: Option<String>,
    pub mongosh_path: Option<String>,
    pub telemetry_enabled: Option<bool>,
    pub skip_update_check: Option<bool>,
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub auth_type: Option<AuthType>,
    pub org_id: Option<String>,
    pub project_id: Option<String>,
    pub service: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum AuthType {
    #[serde(rename = "user_account")]
    UserAccount,
    #[serde(rename = "api_keys")]
    ApiKeys,
    #[serde(rename = "service_account")]
    ServiceAccount,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_00_empty() {
        let config = Config::from_str(include_str!("../../fixtures/00_empty.toml"))
            .expect("Deserialize should succeed");

        assert_eq!(config.version, 2);
        assert_eq!(config.local_deployment_image, None);
        assert_eq!(config.mongosh_path, None);
        assert_eq!(config.telemetry_enabled, None);
        assert_eq!(config.skip_update_check, None);
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_deserialize_01_global_properties() {
        let config = Config::from_str(include_str!("../../fixtures/01_global_properties.toml"))
            .expect("Deserialize should succeed");

        assert_eq!(config.version, 2);
        assert_eq!(
            config.local_deployment_image,
            Some("my-registry.internal:5000/atlas-cli-local:latest".to_string())
        );
        assert_eq!(
            config.mongosh_path,
            Some("/home/user/mdb-tools/mongosh".to_string())
        );
        assert_eq!(config.telemetry_enabled, Some(true));
        assert_eq!(config.skip_update_check, Some(true));
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_deserialize_02_user_account() {
        let config = Config::from_str(include_str!("../../fixtures/02_user_account.toml"))
            .expect("Deserialize should succeed");

        assert_eq!(config.version, 2);

        let profile_1 = config
            .profiles
            .get("profile_1")
            .expect("Profile should exist");
        assert_eq!(profile_1.auth_type, Some(AuthType::UserAccount));
        assert_eq!(
            profile_1.org_id,
            Some("689eeba6559f4608e426b000".to_string())
        );
        assert_eq!(
            profile_1.project_id,
            Some("689eebca5ebb720663a2d123".to_string())
        );
        assert_eq!(profile_1.service, Some("cloud".to_string()));
        assert_eq!(profile_1.output, Some("json".to_string()));
    }

    #[test]
    fn test_deserialize_03_all_options() {
        let config = Config::from_str(include_str!("../../fixtures/03_all_options.toml"))
            .expect("Deserialize should succeed");

        assert_eq!(config.version, 2);

        let profile_with_user_account = config
            .profiles
            .get("profile_with_user_account")
            .expect("Profile should exist");
        assert_eq!(
            profile_with_user_account.auth_type,
            Some(AuthType::UserAccount)
        );
        assert_eq!(
            profile_with_user_account.org_id,
            Some("689eeba6559f4608e426b000".to_string())
        );
        assert_eq!(
            profile_with_user_account.project_id,
            Some("689eebca5ebb720663a2d123".to_string())
        );
        assert_eq!(profile_with_user_account.service, Some("cloud".to_string()));
        assert_eq!(profile_with_user_account.output, Some("json".to_string()));

        let profile_with_api_keys = config
            .profiles
            .get("profile_with_api_keys")
            .expect("Profile should exist");
        assert_eq!(profile_with_api_keys.auth_type, Some(AuthType::ApiKeys));
        assert_eq!(profile_with_api_keys.org_id, None);
        assert_eq!(profile_with_api_keys.project_id, None);
        assert_eq!(profile_with_api_keys.service, None);
        assert_eq!(profile_with_api_keys.output, None);

        let profile_with_service_account = config
            .profiles
            .get("profile_with_service_account")
            .expect("Profile should exist");
        assert_eq!(
            profile_with_service_account.auth_type,
            Some(AuthType::ServiceAccount)
        );
        assert_eq!(profile_with_service_account.org_id, None);
        assert_eq!(profile_with_service_account.project_id, None);
        assert_eq!(
            profile_with_service_account.service,
            Some("cloudgov".to_string())
        );
        assert_eq!(profile_with_service_account.output, Some("txt".to_string()));
    }
}
