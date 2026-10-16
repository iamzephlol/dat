use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Supported data formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Csv,
    Tsv,
    Auto,
}

impl Format {
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => Format::Json,
            Some("yaml") | Some("yml") => Format::Yaml,
            Some("toml") => Format::Toml,
            Some("csv") => Format::Csv,
            Some("tsv") => Format::Tsv,
            _ => Format::Auto,
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Json => write!(f, "JSON"),
            Format::Yaml => write!(f, "YAML"),
            Format::Toml => write!(f, "TOML"),
            Format::Csv => write!(f, "CSV"),
            Format::Tsv => write!(f, "TSV"),
            Format::Auto => write!(f, "Auto"),
        }
    }
}

/// A unified value type that wraps serde_json::Value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

#[allow(dead_code)]
impl Value {
    pub fn is_null(&self) -> bool { matches!(self, Value::Null) }
    pub fn is_bool(&self) -> bool { matches!(self, Value::Bool(_)) }
    pub fn is_number(&self) -> bool { matches!(self, Value::Integer(_) | Value::Float(_)) }
    pub fn is_string(&self) -> bool { matches!(self, Value::String(_)) }
    pub fn is_array(&self) -> bool { matches!(self, Value::Array(_)) }
    pub fn is_object(&self) -> bool { matches!(self, Value::Object(_)) }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if let Value::Object(obj) = self {
            obj.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&[(String, Value)]> {
        if let Value::Object(o) = self { Some(o) } else { None }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(a) = self { Some(a) } else { None }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self { Some(*b) } else { None }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Value::Integer(i) = self { Some(*i) } else { None }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Float(f) = self { Some(*f) } else if let Value::Integer(i) = self { Some(*i as f64) } else { None }
    }

    pub fn keys(&self) -> Vec<String> {
        if let Value::Object(obj) = self {
            obj.iter().map(|(k, _)| k.clone()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Value::Array(arr) => arr.len(),
            Value::Object(obj) => obj.len(),
            _ => 1,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (String, Value)> {
        if let Value::Object(obj) = self { obj.iter() } else { [].iter() }
    }
}

impl Default for Value {
    fn default() -> Self { Value::Null }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self { Value::Integer(i) }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self { Value::Float(f) }
}

impl From<String> for Value {
    fn from(s: String) -> Self { Value::String(s) }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self { Value::String(s.to_string()) }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self { Value::Array(v) }
}

impl From<Vec<(String, Value)>> for Value {
    fn from(v: Vec<(String, Value)>) -> Self { Value::Object(v) }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                let mut iter = obj.iter().peekable();
                while let Some((k, v)) = iter.next() {
                    write!(f, "\"{}\": {}", k, v)?;
                    if iter.peek().is_some() { write!(f, ", ")?; }
                }
                write!(f, "}}")
            }
        }
    }
}

/// View modes for displaying data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Tree,
    Table,
    List,
    Raw,
    Schema,
}

impl std::fmt::Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewMode::Tree => write!(f, "tree"),
            ViewMode::Table => write!(f, "table"),
            ViewMode::List => write!(f, "list"),
            ViewMode::Raw => write!(f, "raw"),
            ViewMode::Schema => write!(f, "schema"),
        }
    }
}

/// jq-like query engine
pub mod query {
    use crate::data::Value;
    /// Query engine for jq-like filtering and sorting
    pub struct QueryEngine;

    impl QueryEngine {
        /// Create a new query engine
        pub fn new() -> Self {
            Self
        }

        /// Apply a jq-like filter query to data
        pub fn query(&self, data: &Value, query: &str) -> anyhow::Result<Value> {
            // Simple query parsing - just extract path references
            // For full jq compatibility, a proper parser would be needed
            let query = query.trim();

            // Check if it's a simple path like ".key" or ".key.subkey"
            if query.starts_with('.') {
                self.evaluate_path(data, &query[1..])
            } else {
                // Try as literal value or return original
                Ok(data.clone())
            }
        }

        fn evaluate_path(&self, data: &Value, path: &str) -> anyhow::Result<Value> {
            let keys: Vec<&str> = path.split('.').collect();
            let mut current = data;

            for key in &keys {
                match current {
                    Value::Object(obj) => {
                        let key_str = *key;
                        current = obj
                            .iter()
                            .find(|(k, _)| k == &key_str)
                            .map(|(_, v)| v)
                            .unwrap_or(&Value::Null);
                    }
                    Value::Array(arr) => {
                        // Try numeric index
                        if let Ok(idx) = key.parse::<usize>() {
                            if idx < arr.len() {
                                current = &arr[idx];
                            } else {
                                return Ok(Value::Null);
                            }
                        } else {
                            return Ok(Value::Null);
                        }
                    }
                    _ => return Ok(Value::Null),
                }
            }

            Ok(current.clone())
        }

        /// Sort an array by a field
        pub fn sort_array(&self, arr: Vec<Value>, by: &str, ascending: bool) -> Vec<Value> {
            let mut sorted = arr;

            sorted.sort_by(|a, b| {
                let a_val = self.extract_sort_value(a, by);
                let b_val = self.extract_sort_value(b, by);

                match (a_val.partial_cmp(&b_val), ascending) {
                    (Some(cmp), true) => cmp,
                    (Some(cmp), false) => cmp.reverse(),
                    _ => std::cmp::Ordering::Equal,
                }
            });

            sorted
        }

        fn extract_sort_value(&self, value: &Value, by: &str) -> f64 {
            let field_name = by.trim();

            // For objects, try to get the field value
            if let Value::Object(obj) = value {
                if let Some(field_val) = obj.iter().find(|(k, _)| k == field_name).map(|(_, v)| v) {
                    return self.value_to_f64(field_val);
                }
            }

            // For other types, try to convert
            self.value_to_f64(value)
        }

        fn value_to_f64(&self, value: &Value) -> f64 {
            match value {
                Value::Integer(i) => *i as f64,
                Value::Float(f) => *f,
                Value::String(s) => {
                    s.parse::<f64>().unwrap_or(0.0)
                }
                Value::Bool(b) => if *b { 1.0 } else { 0.0 },
                _ => 0.0,
            }
        }
    }
}

/// Load data from a file
pub fn load_data(path: &Path, format: Format) -> Result<Value> {
    let content = fs::read_to_string(path)?;

    let value = match format {
        Format::Json => {
            let json: serde_json::Value = serde_json::from_str(&content)?;
            serde_json_to_value(&json)
        }
        Format::Yaml => {
            let json: serde_json::Value = serde_yaml::from_str(&content)?;
            serde_json_to_value(&json)
        }
        Format::Toml => {
            let toml_value: toml::Value = toml::from_str(&content)?;
            toml_to_value(&toml_value)
        }
        Format::Csv => {
            let mut reader = csv::Reader::from_reader(content.as_bytes());
            let mut rows = Vec::new();

            let headers: Vec<String> = reader
                .headers()
                .map(|h| h.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default();

            if !headers.is_empty() {
                for result in reader.records() {
                    let record = result?;
                    let mut obj = Vec::new();
                    for (header, value) in headers.iter().zip(record.iter()) {
                        obj.push((header.clone(), Value::String(value.to_string())));
                    }
                    rows.push(Value::Object(obj));
                }
                Value::Array(rows)
            } else {
                for result in reader.records() {
                    let record = result?;
                    let mut arr = Vec::new();
                    for value in record.iter() {
                        arr.push(Value::String(value.to_string()));
                    }
                    rows.push(Value::Array(arr));
                }
                Value::Array(rows)
            }
        }
        Format::Tsv => {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_reader(content.as_bytes());

            let mut rows = Vec::new();

            let headers: Vec<String> = reader
                .headers()
                .map(|h| h.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default();

            if !headers.is_empty() {
                for result in reader.records() {
                    let record = result?;
                    let mut obj = Vec::new();
                    for (header, value) in headers.iter().zip(record.iter()) {
                        obj.push((header.clone(), Value::String(value.to_string())));
                    }
                    rows.push(Value::Object(obj));
                }
                Value::Array(rows)
            } else {
                for result in reader.records() {
                    let record = result?;
                    let mut arr = Vec::new();
                    for value in record.iter() {
                        arr.push(Value::String(value.to_string()));
                    }
                    rows.push(Value::Array(arr));
                }
                Value::Array(rows)
            }
        }
        Format::Auto => {
            if let Ok(v) = load_data(path, Format::Json) { return Ok(v); }
            if let Ok(v) = load_data(path, Format::Yaml) { return Ok(v); }
            if let Ok(v) = load_data(path, Format::Toml) { return Ok(v); }
            if let Ok(v) = load_data(path, Format::Csv) { return Ok(v); }
            return Err(anyhow::anyhow!("Could not determine file format"));
        }
    };

    Ok(value)
}

fn serde_json_to_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Null
            }
        }
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.iter().map(serde_json_to_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut vec = Vec::new();
            for (k, v) in obj.iter() {
                vec.push((k.clone(), serde_json_to_value(v)));
            }
            Value::Object(vec)
        }
    }
}

fn toml_to_value(v: &toml::Value) -> Value {
    match v {
        toml::Value::String(s) => Value::String(s.clone()),
        toml::Value::Integer(i) => Value::Integer(*i),
        toml::Value::Float(f) => Value::Float(*f),
        toml::Value::Boolean(b) => Value::Bool(*b),
        toml::Value::Array(arr) => {
            Value::Array(arr.iter().map(toml_to_value).collect())
        }
        toml::Value::Table(tbl) => {
            let mut vec = Vec::new();
            for (k, v) in tbl.iter() {
                vec.push((k.clone(), toml_to_value(v)));
            }
            Value::Object(vec)
        }
        toml::Value::Datetime(d) => Value::String(d.to_string()),
    }
}