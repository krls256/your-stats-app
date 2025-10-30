use std::sync::{Arc, Mutex, OnceLock};

use crate::repository::Repository;
use crate::domain::{correlation::Correlation, data_point::{DataPoint}, metric::{Metric}, MetricType};
use crate::domain::metric::MetricWithDataPoints;

static REPO: OnceLock<Arc<Mutex<Repository>>> = OnceLock::new();

pub fn init_db(path: &str) {
    let repo = Repository::new(path).expect("Failed to create database");
    let _ = REPO.set(Arc::new(Mutex::new(repo)));
}

fn db() -> &'static Arc<Mutex<Repository>> {
    REPO.get().expect("DB is not initialized. Call init_db() first")
}

pub fn create_metric(name: &str, metric_type: MetricType, description: Option<&str>) -> anyhow::Result<i64> {
    let lock = db().lock().unwrap();
    lock.create_metric(name, metric_type, description)
}

pub fn list_metrics() -> anyhow::Result<Vec<Metric>> {
    let lock = db().lock().unwrap();
    lock.get_all_metrics()
}

pub fn add_data_point(metric_id: i64, value: &str) -> anyhow::Result<i64> {
    let lock = db().lock().unwrap();
    lock.add_data_point(metric_id, value)
}

pub fn get_data_points_paginated(metric_id: i64, limit: usize, offset: usize) -> anyhow::Result<Vec<DataPoint>> {
    let lock = db().lock().unwrap();
    lock.get_data_points_paginated(metric_id, limit, offset)
}

pub fn get_data_points_count(metric_id: i64) -> anyhow::Result<usize> {
    let lock = db().lock().unwrap();
    lock.get_data_points_count(metric_id)
}

pub fn delete_metric(metric_id: i64) -> anyhow::Result<()> {
    let lock = db().lock().unwrap();
    lock.delete_metric(metric_id)
}

pub fn delete_data_point(point_id: i64) -> anyhow::Result<()> {
    let lock = db().lock().unwrap();
    lock.delete_data_point(point_id)
}

pub fn calculate_correlations() -> anyhow::Result<Vec<Correlation>> {
    let lock = db().lock().unwrap();
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
