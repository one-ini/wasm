//! # EditorConfig
//!
//! A collection of utilities that handle the parsing of
//! [EditorConfig-INI](https://editorconfig-specification.readthedocs.io/en/latest/#file-format)
//! file contents into [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree),
//! which can then be modified and/or serialized.

// Enable WeeAlloc as global memory allocator for the WASM target
#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use one_ini::{parse, parse_to_vec};
use std::{env, str};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_to_json(contents: &str) -> JsValue {
    let ast = parse(&contents).unwrap();
    return serde_wasm_bindgen::to_value(&ast).unwrap();
    //return JsValue::from_serde(&ast).unwrap();
}

#[wasm_bindgen]
pub fn version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum TokenTypes {
    Key,
    Value,
    Section,
    CommentIndicator,
    CommentValue,
}

#[wasm_bindgen]
pub fn parse_to_uint32array(contents: &[u8]) -> Result<Vec<u32>, JsError> {
    let input = str::from_utf8(contents)?;
    match parse_to_vec(input) {
        Ok(res) => Ok(res),
        Err(er) => Err(JsError::from(er)),
    }
}
