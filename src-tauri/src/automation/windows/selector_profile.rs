use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ElementSelectorCandidate {
    pub automation_id: Option<String>,
    pub names: Option<Vec<String>>,
    pub control_type: Option<String>,
    pub class_name: Option<String>,
    pub required_patterns: Option<Vec<String>>,
    pub ancestor_hints: Option<Vec<String>>,
    pub descendant_hints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SelectorTarget {
    pub process_names: Vec<String>,
    pub window_title_hints: Vec<String>,
    pub app_versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SelectorProfile {
    pub schema_version: u32,
    pub profile_id: String,
    pub description: String,
    pub target: SelectorTarget,
    pub selectors: HashMap<String, Vec<ElementSelectorCandidate>>,
}

#[allow(dead_code)]
impl SelectorProfile {
    /// Parses a JSON string into a SelectorProfile and performs strict schema validation.
    pub fn parse(json_str: &str) -> Result<Self, String> {
        let profile: Self = serde_json::from_str(json_str)
            .map_err(|e| format!("JSON deserialization failed: {}", e))?;

        if profile.schema_version != 1 {
            return Err(format!(
                "Unsupported schema version: {}. Expected 1.",
                profile.schema_version
            ));
        }

        if profile.profile_id.trim().is_empty() {
            return Err("Profile ID cannot be empty.".into());
        }

        if profile.target.process_names.is_empty() {
            return Err("Profile target must specify at least one process name.".into());
        }

        Ok(profile)
    }

    /// Determines if this selector profile is compatible with a given target window and version.
    pub fn is_compatible(
        &self,
        process_name: &str,
        window_title: &str,
        app_version: Option<&str>,
    ) -> bool {
        // 1. Process name match (case-insensitive)
        let process_matches = self
            .target
            .process_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(process_name));
        if !process_matches {
            return false;
        }

        // 2. Window title match (case-insensitive substring hint match)
        let title_lower = window_title.to_lowercase();
        let title_matches = self.target.window_title_hints.iter().any(|hint| {
            let hint_lower = hint.to_lowercase();
            title_lower.contains(&hint_lower)
        });
        if !title_matches && !self.target.window_title_hints.is_empty() {
            return false;
        }

        // 3. App version match (if version information is supplied and the profile specifies allowed versions)
        if let Some(version) = app_version {
            if !self.target.app_versions.is_empty() {
                let version_matches = self
                    .target
                    .app_versions
                    .iter()
                    .any(|v| v.eq_ignore_ascii_case(version));
                if !version_matches {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_profile() {
        let data = r#"{
            "schemaVersion": 1,
            "profileId": "chatgpt-v1",
            "description": "Test profile",
            "target": {
                "processNames": ["ChatGPT.exe"],
                "windowTitleHints": ["ChatGPT"],
                "appVersions": ["1.0.0"]
            },
            "selectors": {
                "approve": [
                  {
                    "automationId": "approve-btn",
                    "controlType": "Button"
                  }
                ]
            }
        }"#;

        let parsed = SelectorProfile::parse(data).unwrap();
        assert_eq!(parsed.profile_id, "chatgpt-v1");
        assert_eq!(parsed.target.process_names[0], "ChatGPT.exe");
        assert_eq!(
            parsed.selectors.get("approve").unwrap()[0].automation_id,
            Some("approve-btn".into())
        );
    }

    #[test]
    fn rejects_invalid_schema_version() {
        let data = r#"{
            "schemaVersion": 2,
            "profileId": "chatgpt-v1",
            "description": "Test profile",
            "target": {
                "processNames": ["ChatGPT.exe"],
                "windowTitleHints": ["ChatGPT"],
                "appVersions": []
            },
            "selectors": {}
        }"#;

        let err = SelectorProfile::parse(data).unwrap_err();
        assert!(err.contains("Unsupported schema version"));
    }

    #[test]
    fn rejects_empty_profile_id() {
        let data = r#"{
            "schemaVersion": 1,
            "profileId": " ",
            "description": "Test profile",
            "target": {
                "processNames": ["ChatGPT.exe"],
                "windowTitleHints": [],
                "appVersions": []
            },
            "selectors": {}
        }"#;

        let err = SelectorProfile::parse(data).unwrap_err();
        assert!(err.contains("Profile ID cannot be empty"));
    }

    #[test]
    fn rejects_empty_process_names() {
        let data = r#"{
            "schemaVersion": 1,
            "profileId": "test",
            "description": "Test profile",
            "target": {
                "processNames": [],
                "windowTitleHints": [],
                "appVersions": []
            },
            "selectors": {}
        }"#;

        let err = SelectorProfile::parse(data).unwrap_err();
        assert!(err.contains("specify at least one process name"));
    }

    #[test]
    fn matches_compatible_target() {
        let profile = SelectorProfile {
            schema_version: 1,
            profile_id: "test".into(),
            description: "test".into(),
            target: SelectorTarget {
                process_names: vec!["ChatGPT.exe".into()],
                window_title_hints: vec!["ChatGPT".into()],
                app_versions: vec!["2.0".into()],
            },
            selectors: HashMap::new(),
        };

        assert!(profile.is_compatible("chatgpt.exe", "Main ChatGPT Window", Some("2.0")));
        assert!(!profile.is_compatible("other.exe", "Main ChatGPT Window", Some("2.0")));
        assert!(!profile.is_compatible("chatgpt.exe", "Other Title", Some("2.0")));
        assert!(!profile.is_compatible("chatgpt.exe", "Main ChatGPT Window", Some("1.0")));
    }
}
