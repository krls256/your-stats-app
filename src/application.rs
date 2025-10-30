use std::sync::{Arc, Mutex};
use anyhow::{Result};

use crate::repository::Repository;
use crate::domain::{correlation::Correlation, data_point::{DataPoint}, metric::{Metric}, MetricType};
use crate::domain::metric::MetricWithDataPoints;

#[derive(Clone)]
pub struct Service {
    repo: Arc<Mutex<Repository>>
}

impl Service {
    pub(crate) fn new(path: &str) -> Result<Self> {
        let repo = Repository::new(path)?;

        Ok(Service{
            repo: Arc::new(Mutex::new(repo)),
        })
    }

    pub fn create_metric(&self, name: &str, metric_type: MetricType, description: Option<&str>) -> Result<i64> {
        let lock = self.repo.lock().unwrap();
        lock.create_metric(name, metric_type, description)
    }

    pub fn list_metrics(&self) -> Result<Vec<Metric>> {
        let lock = self.repo.lock().unwrap();
        lock.get_all_metrics()
    }

    pub fn add_data_point(&self, metric_id: i64, value: &str) -> Result<i64> {
        let lock = self.repo.lock().unwrap();
        lock.add_data_point(metric_id, value)
    }

    pub fn get_data_points_paginated(&self, metric_id: i64, limit: usize, offset: usize) -> Result<Vec<DataPoint>> {
        let lock = self.repo.lock().unwrap();

        lock.get_data_points_paginated(metric_id, limit, offset)
    }

    pub fn get_data_points_count(&self, metric_id: i64) -> Result<usize> {
        let lock = self.repo.lock().unwrap();
        lock.get_data_points_count(metric_id)
    }

    pub fn delete_metric(&self, metric_id: i64) -> Result<()> {
        let lock = self.repo.lock().unwrap();
        lock.delete_metric(metric_id)
    }

    pub fn delete_data_point(&self, point_id: i64) -> Result<()> {
        let lock = self.repo.lock().unwrap();
        lock.delete_data_point(point_id)
    }

    pub fn calculate_correlations(&self) -> Result<Vec<Correlation>> {
        let lock = self.repo.lock().unwrap();
        let metrics = lock.get_all_metrics()?;
        let data_points = lock.get_all_data_points()?;

        let metrics_with_data = MetricWithDataPoints::join(metrics, data_points);

        let mut correlations = Vec::new();

        for i in 0..metrics_with_data.len() {
            for j in (i + 1)..metrics_with_data.len() {
                if let Ok(corr) = Correlation::calc(&metrics_with_data[i], &metrics_with_data[j]) {
                    correlations.push(corr)
                }
            }
        }

        Ok(correlations)
    }

}


