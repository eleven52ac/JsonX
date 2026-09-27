//! Direct-children lookup for the Tree View's lazy loading: given a path of
//! object keys / array indices, return just that node's immediate children
//! instead of materializing the whole subtree. Not yet wired to the
//! wasm-bindgen boundary in `lib.rs` — the JS-side fallback in
//! `src/worker/engine.ts` covers this today.

use crate::parser::Value;

pub enum PathSegment {
    Key(String),
    Index(usize),
}

pub struct ChildInfo {
    pub key: Option<String>,
    pub index: Option<usize>,
    pub value_kind: &'static str,
    pub preview: String,
    pub child_count: usize,
}

pub fn children_at(root: &Value, path: &[PathSegment]) -> Vec<ChildInfo> {
    let target = match resolve(root, path) {
        Some(v) => v,
        None => return Vec::new(),
    };
    match target {
        Value::Array(items) => items
            .iter()
            .enumerate()
            .map(|(i, v)| ChildInfo {
                key: None,
                index: Some(i),
                value_kind: kind_name(v),
                preview: preview_of(v),
                child_count: child_count(v),
            })
            .collect(),
        Value::Object(entries) => entries
            .iter()
            .map(|(k, v)| ChildInfo {
                key: Some(k.clone()),
                index: None,
                value_kind: kind_name(v),
                preview: preview_of(v),
                child_count: child_count(v),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn resolve<'a>(root: &'a Value, path: &[PathSegment]) -> Option<&'a Value> {
    let mut current = root;
    for segment in path {
        let next = match (current, segment) {
            (Value::Object(entries), PathSegment::Key(key)) => {
                let mut found = None;
                for (k, v) in entries {
                    if k == key {
                        found = Some(v);
                        break;
                    }
                }
                found
            }
            (Value::Array(items), PathSegment::Index(i)) => items.get(*i),
            _ => None,
        };
        current = next?;
    }
    Some(current)
}

fn kind_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn child_count(value: &Value) -> usize {
    match value {
        Value::Array(items) => items.len(),
        Value::Object(entries) => entries.len(),
        _ => 0,
    }
}

fn preview_of(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => crate::formatter::format_number(*n),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(items) => format!("Array[{}]", items.len()),
        Value::Object(entries) => format!("Object{{{}}}", entries.len()),
    }
}
