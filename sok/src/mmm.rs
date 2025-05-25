// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_b(a:u32,b:u32)->u32{
    return  a+b;
}
