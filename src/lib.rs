mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn req() -> String {
     let resp = match minreq::get("http://httpbin.org/ip").send() {
         Ok(r) => return format!("{:?}", r),
         Err(e) => return format!("Grr {:?}", e),
     };
}
