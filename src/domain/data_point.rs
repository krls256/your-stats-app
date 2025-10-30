use chrono::{DateTime, ParseError, Utc};
use crate::domain::readable_datetime;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub id: i64,
    pub metric_id: i64,
    pub value: String,
    pub timestamp: DateTime<Utc>,
}

impl DataPoint {
    pub fn readable_timestamp(&self) -> String {
        readable_datetime(self.timestamp)
    }

    pub fn try_f64(&self) -> Result<f64> {
        if let Ok(v) = self.value.parse::<f64>() {
            Ok(v)
        } else {
            let low = self.value.to_lowercase();

            match low.as_str() {
                "true" => Ok(1.0),
                "false" => Ok(0.0),
                _ => Err(anyhow!("can't parse f64"))
            }
        }
    }
}

impl TryFrom<DataPointDTO> for DataPoint {
    type Error = ParseError;

    fn try_from(value: DataPointDTO) -> Result<Self, Self::Error> {
        let DataPointDTO {
            id,
            metric_id,
            value,
            timestamp,
        } = value;

        Ok(DataPoint {
            id,
            metric_id,
            value,
            timestamp: DateTime::parse_from_rfc3339(timestamp.as_str())?.with_timezone(&Utc),
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DataPointDTO {
    pub id: i64,
    pub metric_id: i64,
    pub value: String,
    pub timestamp: String,
}
