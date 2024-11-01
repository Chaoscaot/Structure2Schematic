mod utils;

use std::convert::TryFrom;
use js_sys::{Array, JsString, Uint8Array};
use wasm_bindgen::prelude::*;
use web_sys::File;
use s2s_core::formats::schematic::SpongeSchematic;
use s2s_core::formats::structure::MinecraftStructure;

#[wasm_bindgen]
pub fn convert_structure(data: &Uint8Array) -> Result<Uint8Array, JsValue> {
    let binary = data.to_vec();

    let structure = MinecraftStructure::try_from(binary).map_err(|e| JsString::from("Cannot read Structure"))?;

    let schem = SpongeSchematic::from(structure);

    let bytes = schem.to_bytes().map_err(|err| JsString::from(err.to_string()))?;

    Ok(Uint8Array::from(bytes.as_slice()))
}
