//! Canonical identifiers and boundary values independent of external systems.

/// Stable identifier for an adapter capability expressed in Melly terms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Creates a capability identifier after checking its basic shape.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidCapabilityId> {
        let value = value.into();
        let valid = !value.is_empty()
            && value
                .split('.')
                .all(|segment| !segment.is_empty() && segment.bytes().all(is_identifier_byte));

        valid.then_some(Self(value)).ok_or(InvalidCapabilityId)
    }

    /// Returns the canonical dotted identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'_'
}

/// A capability identifier was empty or malformed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidCapabilityId;

impl std::fmt::Display for InvalidCapabilityId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("capability identifiers must contain dotted lowercase segments")
    }
}

impl std::error::Error for InvalidCapabilityId {}

#[cfg(test)]
mod tests {
    use super::CapabilityId;

    #[test]
    fn accepts_host_neutral_capability_names() {
        assert_eq!(
            CapabilityId::new("workspaces.control").unwrap().as_str(),
            "workspaces.control"
        );
    }

    #[test]
    fn rejects_empty_segments_and_native_names() {
        assert!(CapabilityId::new("workspaces..control").is_err());
        assert!(CapabilityId::new("Sway.command").is_err());
    }
}
