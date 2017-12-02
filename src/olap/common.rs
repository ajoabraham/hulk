extern crate chrono;

use olap::chrono::prelude::*;

#[derive(Debug)]
pub enum DataType {
    INT,
    BIGINT,
    DECIMAL,
    FLOAT,
    BOOL,
    DATE,
    DATETIME,
    TEXT
}

#[derive(Debug)]
pub enum Record{
    INT(i32),
    BIGINT(i64),
    DECIMAL(f64),
    FLOAT(f32),
    BOOL(bool),
    DATE(Date<Utc>),
    DATETIME(DateTime<Utc>),
    TEXT(String)
}

#[derive(Debug)]
pub enum ColType {
    DIMENSION,
    MEASURE
}

#[derive(Debug)]
pub enum AggMode {
    SUM,
    AVG,
    MIN,
    MAX,
    COUNT
}

