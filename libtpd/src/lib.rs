#[macro_use]
pub mod macros;
pub mod parser;
pub mod syntax;
pub mod types;
pub mod vm;
pub mod compiler;
pub mod buildin;
pub mod logger;

#[cfg(target_arch = "wasm32")]
pub mod web;