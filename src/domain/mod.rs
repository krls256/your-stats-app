pub mod metric;
pub mod data_point;
pub mod correlation;

use chrono::{DateTime, Utc};
use strum_macros::{EnumIter, FromRepr};
use strum::{IntoEnumIterator};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumIter, FromRepr)]
pub enum MetricType {
    Integer,
    #[default]
    Float,
    Boolean,
}

impl MetricType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetricType::Integer => "INTEGER",
            MetricType::Float => "FLOAT",
            MetricType::Boolean => "BOOLEAN",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INTEGER" => Some(MetricType::Integer),
            "FLOAT" => Some(MetricType::Float),
            "BOOLEAN" => Some(MetricType::Boolean),
            _ => None,
        }
    }

    pub fn all() -> impl Iterator<Item=Self> {
        MetricType::iter().rev()
    }
}

pub fn readable_datetime(t: DateTime<Utc>) -> String {
    t.format("%a, %d %b %Y %H:%M:%S").to_string()
}


pub fn offset(page: usize, limit: usize) -> usize {
    (page - 1) * limit
}