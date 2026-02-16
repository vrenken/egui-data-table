use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relation {
    pub source: String,
    pub key: String,
    pub value: String,
}

impl Relation {
    pub fn new(source: impl Into<String>, key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "relation://{}/{}/{}", self.source, self.key, self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelationParseError {
    InvalidFormat,
    MissingPrefix,
}

impl fmt::Display for RelationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationParseError::InvalidFormat => write!(f, "Invalid relation format"),
            RelationParseError::MissingPrefix => write!(f, "Relation must start with relation://"),
        }
    }
}

impl std::error::Error for RelationParseError {}

impl FromStr for Relation {
    type Err = RelationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("relation://").ok_or(RelationParseError::MissingPrefix)?;
        let parts: Vec<&str> = stripped.splitn(3, '/').collect();
        
        if parts.len() != 3 {
            return Err(RelationParseError::InvalidFormat);
        }

        Ok(Relation {
            source: parts[0].to_string(),
            key: parts[1].to_string(),
            value: parts[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation_serialization() {
        let relation = Relation::new("src", "k", "v");
        assert_eq!(relation.to_string(), "relation://src/k/v");
    }

    #[test]
    fn test_relation_deserialization() {
        let s = "relation://src/k/v";
        let relation: Relation = s.parse().unwrap();
        assert_eq!(relation, Relation::new("src", "k", "v"));
    }

    #[test]
    fn test_relation_deserialization_invalid() {
        let s = "invalid://src/k/v";
        let result: Result<Relation, _> = s.parse();
        assert_eq!(result.unwrap_err(), RelationParseError::MissingPrefix);

        let s = "relation://src/k";
        let result: Result<Relation, _> = s.parse();
        assert_eq!(result.unwrap_err(), RelationParseError::InvalidFormat);
    }
}
