//! wasm-bindgen boundary for the JSON engine. Everything here operates on
//! whole strings today (normal-mode file sizes); the streaming, chunk-fed
//! path for Large File Mode is expected to land as additional exports here
//! once `tokenizer`/`indexer` grow the incremental APIs the product spec
//! calls for, without changing the shape of the exports below.

mod formatter;
mod indexer;
mod minifier;
mod parser;
mod statistics;
mod tokenizer;
mod validator;

use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_json(input: &str) -> String {
    match validator::validate(input) {
        Ok(()) => "{\"valid\":true}".to_string(),
        Err(e) => {
            let mut message = String::new();
            formatter::write_string(&e.message, &mut message);
            format!(
                "{{\"valid\":false,\"message\":{},\"line\":{},\"column\":{},\"offset\":{}}}",
                message, e.line, e.column, e.offset
            )
        }
    }
}

#[wasm_bindgen]
pub fn format_json(input: &str) -> Result<String, JsValue> {
    let mut parser = Parser::new(input);
    let value = parser.parse().map_err(|e| JsValue::from_str(&e.message))?;
    Ok(formatter::format(&value, 2))
}

#[wasm_bindgen]
pub fn minify_json(input: &str) -> Result<String, JsValue> {
    let mut parser = Parser::new(input);
    let value = parser.parse().map_err(|e| JsValue::from_str(&e.message))?;
    Ok(minifier::minify(&value))
}

#[wasm_bindgen]
pub fn stats_json(input: &str) -> Result<String, JsValue> {
    let mut parser = Parser::new(input);
    let value = parser.parse().map_err(|e| JsValue::from_str(&e.message))?;
    let stats = statistics::collect(&value);
    Ok(format!(
        "{{\"sizeBytes\":{},\"root\":\"{}\",\"objects\":{},\"arrays\":{},\"maxDepth\":{}}}",
        input.len(),
        stats.root,
        stats.objects,
        stats.arrays,
        stats.max_depth
    ))
}
