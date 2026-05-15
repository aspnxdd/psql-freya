use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
use serde_json::Value;
use std::net::IpAddr;
use tokio_postgres::{types::Type, Row};

pub fn parse_value(row: &Row, index: usize, col_type: &Type) -> String {
    match col_type.kind() {
        tokio_postgres::types::Kind::Array(elem_type) => parse_array(row, index, elem_type),
        _ => parse_scalar(row, index, col_type),
    }
}

fn parse_scalar(row: &Row, index: usize, col_type: &Type) -> String {
    match col_type.name() {
        "int2" => row
            .get::<_, Option<i16>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "int4" => row
            .get::<_, Option<i32>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "int8" => row
            .get::<_, Option<i64>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "float4" => row
            .get::<_, Option<f32>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "float8" => row
            .get::<_, Option<f64>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "numeric" => row
            .get::<_, Option<Decimal>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "bool" => row
            .get::<_, Option<bool>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "text" | "varchar" | "char" | "bpchar" | "name" => row
            .get::<_, Option<String>>(index)
            .unwrap_or_else(|| "NULL".to_string()),
        "bytea" => row
            .get::<_, Option<Vec<u8>>>(index)
            .map(|v| format!("\\x{}", bytes_to_hex(&v)))
            .unwrap_or_else(|| "NULL".to_string()),
        "timestamp" => row
            .get::<_, Option<NaiveDateTime>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "timestamptz" => row
            .get::<_, Option<DateTime<FixedOffset>>>(index)
            .map(|v| v.to_rfc3339())
            .unwrap_or_else(|| "NULL".to_string()),
        "date" => row
            .get::<_, Option<NaiveDate>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "time" => row
            .get::<_, Option<NaiveTime>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "uuid" => row
            .get::<_, Option<uuid::Uuid>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "json" | "jsonb" => row
            .get::<_, Option<Value>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "inet" => row
            .get::<_, Option<IpAddr>>(index)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string()),
        "timetz" | "interval" | "cidr" => {
            format!("<unsupported: {}>", col_type.name())
        }
        _ => row
            .get::<_, Option<String>>(index)
            .unwrap_or_else(|| format!("<unsupported: {}>", col_type.name())),
    }
}

fn parse_array(row: &Row, index: usize, elem_type: &Type) -> String {
    if matches!(elem_type.kind(), tokio_postgres::types::Kind::Array(_)) {
        return format!("<unsupported: multi-dimensional array>");
    }

    match elem_type.name() {
        "int2" => format_array(row, index, |v: i16| v.to_string()),
        "int4" => format_array(row, index, |v: i32| v.to_string()),
        "int8" => format_array(row, index, |v: i64| v.to_string()),
        "float4" => format_array(row, index, |v: f32| v.to_string()),
        "float8" => format_array(row, index, |v: f64| v.to_string()),
        "numeric" => format_array(row, index, |v: Decimal| v.to_string()),
        "bool" => format_array(row, index, |v: bool| v.to_string()),
        "text" | "varchar" | "char" | "bpchar" | "name" => format_array(row, index, |v: String| v),
        "timestamp" => format_array(row, index, |v: NaiveDateTime| v.to_string()),
        "timestamptz" => format_array(row, index, |v: DateTime<FixedOffset>| v.to_rfc3339()),
        "date" => format_array(row, index, |v: NaiveDate| v.to_string()),
        "time" => format_array(row, index, |v: NaiveTime| v.to_string()),
        "uuid" => format_array(row, index, |v: uuid::Uuid| v.to_string()),
        "json" | "jsonb" => format_array(row, index, |v: Value| v.to_string()),
        "inet" => format_array(row, index, |v: IpAddr| v.to_string()),
        _ => format!("<unsupported: {}[]>", elem_type.name()),
    }
}

fn format_array<T>(row: &Row, index: usize, fmt: impl Fn(T) -> String) -> String
where
    for<'a> T: tokio_postgres::types::FromSql<'a>,
{
    let arr: Option<Vec<Option<T>>> = row.get(index);
    match arr {
        None => "NULL".to_string(),
        Some(values) => {
            let formatted: Vec<String> = values
                .into_iter()
                .map(|opt| opt.map(&fmt).unwrap_or_else(|| "NULL".to_string()))
                .collect();
            format!("{{{}}}", formatted.join(","))
        }
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}
