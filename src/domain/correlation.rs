use anyhow::{anyhow, Result};
use crate::domain::metric::{MetricWithDataPoints};
use crate::domain::MetricType;

#[derive(Debug, Clone, PartialEq)]
pub struct Correlation {
    pub metric1_id: i64,
    pub metric1_name: String,
    pub metric1_type: MetricType,
    pub metric2_id: i64,
    pub metric2_name: String,
    pub metric2_type: MetricType,
    pub correlation: f64,
    pub count: usize,
}

impl Correlation {
    pub fn calc(m1: &MetricWithDataPoints, m2: &MetricWithDataPoints) -> Result<Self> {
        let v1 = m1.f64_series()?;
        let v2 = m2.f64_series()?;

        let count = v1.len().min(v2.len());

        let corr = pearson(&v1[..count], &v2[..count])?;
        Ok(Correlation {
            metric1_id: m1.metric.id,
            metric1_name: m1.metric.name.clone(),
            metric1_type: m1.metric.metric_type,
            metric2_id: m2.metric.id,
            metric2_name: m2.metric.name.clone(),
            metric2_type: m2.metric.metric_type,
            correlation: corr,
            count,
        })
    }

    pub fn correlation_type(&self) -> CorrelationType {
        let abs = self.correlation.abs();

        if abs > 0.7 {
            CorrelationType::Strong
        } else if abs > 0.4 {
            CorrelationType::Medium
        } else {
            CorrelationType::Weak
        }
    }
}

fn pearson(x: &[f64], y: &[f64]) -> anyhow::Result<f64> {
    if x.len() != y.len() || x.len() < 2 {
        return Err(anyhow!("not enough data"));
    }

    let len = x.len();
    let n = len as f64;

    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;

    let mut num = 0.0;

    let mut den_x = 0.0;
    let mut den_y = 0.0;

    for k in 0..len {
        let dx = x[k] - mx;
        let dy = y[k] - my;

        num += dx * dy;
        den_x += dx * dx;
        den_y += dy * dy;
    }

    let den = (den_x * den_y).sqrt();
    if den == 0.0 { return Ok(0.0); }

    Ok(num / den)
}


pub enum CorrelationType {
    Strong,
    Medium,
    Weak,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pearson_returns_1_for_identical_series() {
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = [1.0, 2.0, 3.0, 4.0];

        let c = pearson(&x, &y).unwrap();

        assert!((c - 1.0).abs() < 0.0000001);
    }

    #[test]
    fn pearson_returns_minus_1_for_reverse_series() {
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = [4.0, 3.0, 2.0, 1.0];

        let c = pearson(&x, &y).unwrap();

        assert!((c + 1.0).abs() < 0.0000001);
    }

    #[test]
    fn pearson_returns_0_for_zero_variance() {
        let x = [1.0, 1.0, 1.0];
        let y = [2.0, 3.0, 4.0];

        let c = pearson(&x, &y).unwrap();

        assert!(c.abs() < 0.0000001);
    }

    #[test]
    fn pearson_errors_on_mismatched_or_small_input() {
        let x = [1.0];
        let y = [1.0];
        assert!(pearson(&x, &y).is_err());

        let x2 = [1.0, 2.0, 3.0];
        let y2 = [1.0, 2.0];
        assert!(pearson(&x2, &y2).is_err());
    }

    #[test]
    fn correlation_type_thresholds() {
        let base = Correlation {
            metric1_id: 1,
            metric1_name: "A".to_string(),
            metric1_type: MetricType::Float,
            metric2_id: 2,
            metric2_name: "B".to_string(),
            metric2_type: MetricType::Float,
            correlation: 0.0,
            count: 10,
        };

        let mut c = base.clone();
        c.correlation = 0.7;
        assert!(matches!(c.correlation_type(), CorrelationType::Medium));

        c.correlation = 0.71;
        assert!(matches!(c.correlation_type(), CorrelationType::Strong));

        c.correlation = 0.4;
        assert!(matches!(c.correlation_type(), CorrelationType::Weak));

        c.correlation = 0.41;
        assert!(matches!(c.correlation_type(), CorrelationType::Medium));

        c.correlation = -0.9;
        assert!(matches!(c.correlation_type(), CorrelationType::Strong));
    }
}