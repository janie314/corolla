/// This file contains methods for both spec versions and versions used by an instance of a Corolla DB.
use std::{cmp::Ordering, ops::Deref};

/// A general version type.
/// Uses [this Rust trick](https://stackoverflow.com/a/25415289).
pub struct Version(Vec<u64>);

impl Deref for Version {
    type Target = Vec<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.iter().zip(other) {
            if a < b {
                return Ordering::Less;
            }
            if a > b {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(&self.cmp(&other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.iter().zip(other) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        self.0
            .into_iter()
            .fold("".to_string(), |a, b| format!("{a}.{b}"))
    }
}

impl From<String> for Version {
    fn from(value: String) -> Self {
        value
            .to_string()
            .split('.')
            .map(|i| i.parse::<u64>().unwrap_or_default())
            .collect()
    }
}

impl<const N: usize> From<[u64; N]> for Version {
    fn from(value: [u64; N]) -> Self {
        Version(value.to_vec())
    }
}

/// The version of a spec.json schema. Will only be changed by an update to the Corolla codebase.
pub type SpecVersion = Version;

/// The version of a particular instance of a Corolla DB. Used to decide whether or not conversions in the instance's spec.json should be run.
pub type InstanceVersion = Version;
