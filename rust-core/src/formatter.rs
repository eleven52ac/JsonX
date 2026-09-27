//! Pretty-printer for `Value`. Kept separate from `minifier` even though
//! both walk the same tree, matching the module boundaries described in the
//! product spec (formatter/minifier/validator as distinct responsibilities).

use crate::parser::Value;

pub fn format(value: &Value, indent: usize) -> String {
    let mut out = String::new();
    write_value(value, indent, 0, &mut out);
    out
}

fn write_value(value: &Value, indent: usize, depth: usize, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&format_number(*n)),
        Value::String(s) => write_string(s, out),
        Value::Array(items) => write_array(items, indent, depth, out),
        Value::Object(entries) => write_object(entries, indent, depth, out),
    }
}

fn write_array(items: &[Value], indent: usize, depth: usize, out: &mut String) {
    if items.is_empty() {
        out.push_str("[]");
        return;
    }
    out.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('\n');
        push_indent(out, indent, depth + 1);
        write_value(item, indent, depth + 1, out);
    }
    out.push('\n');
    push_indent(out, indent, depth);
    out.push(']');
}

fn write_object(entries: &[(String, Value)], indent: usize, depth: usize, out: &mut String) {
    if entries.is_empty() {
        out.push_str("{}");
        return;
    }
    out.push('{');
    for (i, (key, value)) in entries.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('\n');
        push_indent(out, indent, depth + 1);
        write_string(key, out);
        out.push_str(": ");
        write_value(value, indent, depth + 1, out);
    }
    out.push('\n');
    push_indent(out, indent, depth);
    out.push('}');
}

fn push_indent(out: &mut String, indent: usize, depth: usize) {
    for _ in 0..(indent * depth) {
        out.push(' ');
    }
}

pub fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

pub fn format_number(n: f64) -> String {
    if n.is_finite() && n == n.trunc() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}
