pub struct Config {

}


// This will only work after step 0 is finished
#[cfg(feature = "step_0")]
use std::{str::FromStr};
// Implement FromStr for Config using toml deserialization
#[cfg(feature = "step_0")]
impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 0: this test should pass when deserialize is implemented
    // See: https://serde.rs/#data-structures
    //
    // Enable this test by adding the `step_0` feature to the default features section in `Cargo.toml`
    #[cfg(feature = "step_0")]
    #[test]
    fn test_deserialize_00_empty() {
        Config::from_str(include_str!("../../fixtures/00_empty.toml"))
            .expect("Deserialize should succeed");
    }

    // Step 1: this test should pass when global properties are added
    //
    // Enable this test by adding the `step_1` feature to the default features section in `Cargo.toml`
    #[cfg(feature = "step_1")]
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
    }

    // Step 2: this test should pass when profiles are added
    // - Which data structure should be used to represent profiles?
    //     - Which data type are we going to use to represent AuthType?
    // - Which field attributes should we use on profile?
    //   See: https://serde.rs/field-attrs.html
    //
    //   Hint, the config file looks like this:
    //   ```plain
    //   property_1 = "value_1"
    //   property_2 = "value_2"
    //   profile_1 = { property_1 = "value_1", property_2 = "value_2" }
    //   ```
    //
    //   Profiles are properties with a complex data type (not string, number, boolean, etc.)
    //   But each property can be a string, number, boolean, etc.
    //
    //   The solution is on the bottom of the file.
    //
    // Enable this test by adding the `step_2` feature to the default features section in `Cargo.toml`
    #[cfg(feature = "step_2")]
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

    // Step 2: this test should pass when profiles are added
    //
    // Enable this test by adding the `step_2` feature to the default features section in `Cargo.toml`
    #[cfg(feature = "step_2")]
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

// Required field attribute for profile:
// #[serde(flatten)]
// pub profiles: HashMap<String, Profile>,