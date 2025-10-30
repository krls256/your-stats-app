use std::collections::HashMap;
use chrono::{DateTime, ParseError, Utc};
use crate::domain::data_point::DataPoint;
use crate::domain::{MetricType, readable_datetime};
use anyhow::{Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    pub id: i64,
    pub name: String,
    pub metric_type: MetricType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Metric {
    pub fn readable_timestamp(&self) -> String {
        readable_datetime(self.created_at)
    }
}
impl TryFrom<MetricDTO> for Metric {
    type Error = ParseError;

    fn try_from(value: MetricDTO) -> Result<Self, Self::Error> {
        let MetricDTO {
            id,
            name,
            metric_type,
            description,
            created_at,
        } = value;

        Ok(Metric {
            id,
            name,
            metric_type,
            description,
            created_at: DateTime::parse_from_rfc3339(created_at.as_str())?.with_timezone(&Utc),
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MetricDTO {
    pub id: i64,
    pub name: String,
    pub metric_type: MetricType,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Clone)]
pub struct MetricWithDataPoints {
    pub metric: Metric,
    pub data_points: Vec<DataPoint>
}

impl MetricWithDataPoints {
    pub fn join(metrics: Vec<Metric>, data_points: Vec<DataPoint>) -> Vec<MetricWithDataPoints> {
        let mut join_map = HashMap::with_capacity(metrics.len());

        for metric in metrics {
            join_map.insert(metric.id, MetricWithDataPoints::new_no_data_points(metric));
        }

        for point in data_points {
            let metric = join_map.get_mut(&point.metric_id);

            if let Some(metric) = metric {
                metric.add_data_point(point)
            }
        }

        join_map.values().cloned().collect()
    }

    pub fn new_no_data_points(metric: Metric) -> MetricWithDataPoints {
        MetricWithDataPoints{
            metric,
            data_points: vec![]
        }
    }
    pub fn f64_series(&self) -> Result<Vec<f64>> {
        self.data_points.iter().map(|p| p.try_f64()).collect()
    }

    pub fn add_data_point(&mut self, data_point: DataPoint) {
        self.data_points.push(data_point)
    }
}
