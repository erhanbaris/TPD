mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

#[cfg(feature = "wasmBuild")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasmBuild")]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[cfg(feature = "wasmBuild")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


#[cfg(not(feature = "wasmBuild"))]
fn main() {
    vm::executer::code_executer(&r#"erhan=2
erhan-=100
erhan *1.3123"#.to_string());
}