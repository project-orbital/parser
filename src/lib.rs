use itertools::Itertools;
use wasm_bindgen::prelude::*;
use implementations::dbs;

mod utils;
mod implementations;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse(array: JsValue) -> String {
    let elements: Vec<String> = array.into_serde().unwrap();
    elements.into_iter().map(dbs::parse).intersperse("\n\n".into()).collect()
}
