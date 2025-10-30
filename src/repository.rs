use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use rusqlite::types::Type;
use crate::domain::{
    MetricType,
    metric::{Metric, MetricDTO},
    data_point::{DataPointDTO, DataPoint},
};

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;

        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                metric_type TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS data_points (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                metric_id INTEGER NOT NULL REFERENCES metrics(id) ON DELETE CASCADE,
                value TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );
            "#,
        )?;

        Ok(())
    }

    pub fn create_metric(&self, name: &str, metric_type: MetricType, description: Option<&str>) -> Result<i64> {
        let created_at: DateTime<Utc> = Utc::now();
        self.conn.execute(
            "INSERT INTO metrics(name, metric_type, description, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![name, metric_type.as_str(), description, created_at.to_rfc3339()],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_all_metrics(&self) -> Result<Vec<Metric>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, metric_type, description, created_at FROM metrics ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map([], |row| Self::row_to_metric(row))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?.try_into()?);
        }

        Ok(out)
    }

    pub fn add_data_point(&self, metric_id: i64, value: &str) -> Result<i64> {
        let ts: DateTime<Utc> = Utc::now();
        self.conn.execute(
            "INSERT INTO data_points(metric_id, value, timestamp) VALUES (?1, ?2, ?3)",
            params![metric_id, value, ts.to_rfc3339()],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_data_points(&self, metric_id: i64, limit: usize) -> Result<Vec<DataPoint>> {
        let mut out = Vec::new();
        let mut stmt = self.conn.prepare(
            "SELECT id, metric_id, value, timestamp FROM data_points WHERE metric_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![metric_id, limit as i64], |row| Self::row_to_datapoint(row))?;
        for r in rows { out.push(r?.try_into()?) }

        Ok(out)
    }

    pub fn get_all_data_points(&self) -> Result<Vec<DataPoint>> {
        let mut out = Vec::new();

        let mut stmt = self.conn.prepare(
            "SELECT id, metric_id, value, timestamp FROM data_points ORDER BY id DESC",
        )?;

        let rows = stmt.query_map(params![], |row| Self::row_to_datapoint(row))?;
        for r in rows { out.push(r?.try_into()?) }

        Ok(out)
    }

    pub fn get_data_points_paginated(&self, metric_id: i64, limit: usize, offset: usize) -> Result<Vec<DataPoint>> {
        let mut out = Vec::new();
        let mut stmt = self.conn.prepare(
            "SELECT id, metric_id, value, timestamp FROM data_points WHERE metric_id = ?1 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
        )?;

        let rows = stmt.query_map(params![metric_id, limit as i64, offset as i64], |row| Self::row_to_datapoint(row))?;
        for r in rows { out.push(r?.try_into()?) }

        Ok(out)
    }

    pub fn get_data_points_count(&self, metric_id: i64) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM data_points WHERE metric_id = ?1",
            params![metric_id],
            |row| row.get(0),
        )?;

        Ok(count as usize)
    }

    pub fn delete_metric(&self, metric_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM metrics WHERE id = ?1",
            params![metric_id],
        )?;

        Ok(())
    }

    pub fn delete_data_point(&self, point_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM data_points WHERE id = ?1",
            params![point_id],
        )?;

        Ok(())
    }

    fn row_to_metric(row: &Row<'_>) -> rusqlite::Result<MetricDTO> {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let mtype_str: String = row.get(2)?;
        let description: Option<String> = row.get(3)?;
        let created_at: String = row.get(4)?;
        let metric_type = MetricType::from_str(&mtype_str)
            .ok_or_else(|| rusqlite::Error::InvalidColumnType(2, "metric_type".into(), Type::Text))?;

        Ok(MetricDTO { id, name, metric_type, description, created_at })
    }

    fn row_to_datapoint(row: &Row<'_>) -> rusqlite::Result<DataPointDTO> {
        Ok(DataPointDTO {
            id: row.get(0)?,
            metric_id: row.get(1)?,
            value: row.get(2)?,
            timestamp: row.get(3)?,
        })
    }
}

