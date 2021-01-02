extern crate bitflags;
extern crate termion;
extern crate log_update;

mod macros;
mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod buildin;

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
    let result = vm::executer::code_executer(&r#"
fn test_1:
    döndür 'erhan'
erhan = test_1
barış = erhan
hataayıklama::doğrula(barış() + " barış", 'erhan barış')
gç::satıryaz(barış(), erhan())
barış
"#.to_string());
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}

