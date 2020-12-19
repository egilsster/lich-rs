use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::types::{DependencyList, DependencySet};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub license: Option<String>,

    #[serde(default)]
    pub dependencies: DependencySet,

    #[serde(default)]
    pub dev_dependencies: DependencySet,

    #[serde(default)]
    pub peer_dependencies: DependencySet,

    #[serde(default)]
    pub bundled_dependencies: DependencyList,

    #[serde(default)]
    pub optional_dependencies: DependencySet,
}

impl Package {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read(path.as_ref())?;
        Self::from_slice(content.as_slice())
    }

    pub fn from_slice(v: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(v)?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::Package;

    #[test]
    fn test_package() {
        let package = Package {
            license: Some("MIT".to_string()),
            dependencies: BTreeMap::new(),
            dev_dependencies: BTreeMap::new(),
            peer_dependencies: BTreeMap::new(),
            optional_dependencies: BTreeMap::new(),
            bundled_dependencies: Vec::new(),
        };

        assert_eq!(Some("MIT".to_string()), package.license);
    }
}
