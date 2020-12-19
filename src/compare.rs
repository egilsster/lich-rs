use crate::types::{ComparisonResult, DependencySet};

pub fn compare_licenses(
    used_dependencies: DependencySet,
    approved_dependencies: Vec<String>,
) -> ComparisonResult {
    let mut results = ComparisonResult::new();

    for (name, _) in used_dependencies.iter() {
        if approved_dependencies.contains(name) {
            results.dependencies.insert(name.to_string(), true);
        } else {
            results.dependencies.insert(name.to_string(), false);
            results.is_ok = false;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencySet;

    #[test]
    fn test_compare_licenses_ok() {
        let mut used_dependencies = DependencySet::new();
        used_dependencies.insert("react".to_string(), "17.0.0".to_string());
        let approved_dependencies = vec!["react".to_string(), "svelte-kit".to_string()];

        assert_eq!(
            true,
            compare_licenses(used_dependencies, approved_dependencies).is_ok
        );
    }

    #[test]
    fn test_compare_licenses_not_ok() {
        let mut used_dependencies = DependencySet::new();
        used_dependencies.insert("react".to_string(), "17.0.0".to_string());
        used_dependencies.insert("lodash".to_string(), "4.17.0".to_string());
        let approved_dependencies = vec!["react".to_string(), "svelte-kit".to_string()];

        assert_eq!(
            false,
            compare_licenses(used_dependencies, approved_dependencies).is_ok
        );
    }
}
