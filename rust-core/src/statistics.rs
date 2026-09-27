//! Structural statistics (object/array counts, max depth) for the info
//! panel. Walks a `Value` tree today; the large-file path should compute
//! these while tokenizing instead of after building a tree, so this module
//! is the one most likely to grow a second, streaming implementation.

use crate::parser::Value;

pub struct Statistics {
    pub root: &'static str,
    pub objects: usize,
    pub arrays: usize,
    pub max_depth: usize,
}

pub fn collect(value: &Value) -> Statistics {
    let mut objects = 0;
    let mut arrays = 0;
    let mut max_depth = 0;
    walk(value, 1, &mut objects, &mut arrays, &mut max_depth);
    Statistics { root: root_kind(value), objects, arrays, max_depth }
}

fn walk(value: &Value, depth: usize, objects: &mut usize, arrays: &mut usize, max_depth: &mut usize) {
    if depth > *max_depth {
        *max_depth = depth;
    }
    match value {
        Value::Array(items) => {
            *arrays += 1;
            for item in items {
                walk(item, depth + 1, objects, arrays, max_depth);
            }
        }
        Value::Object(entries) => {
            *objects += 1;
            for (_, v) in entries {
                walk(v, depth + 1, objects, arrays, max_depth);
            }
        }
        _ => {}
    }
}

fn root_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
