use crate::compiler::{BramaCompiler, function::{NativeCallResult, NativeCall}};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::{Module, Class};
use std::collections::HashMap;
use std::sync::Arc;

pub struct NumModule {
    methods: HashMap<String, NativeCall>
}

impl Module for NumModule {
    fn new() -> NumModule where Self: Sized {
        let mut module = NumModule {
            methods: HashMap::new()
        };
        module.methods.insert("oku".to_string(), Self::parse as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        "sayı".to_string()
    }

    fn get_method(&self, name: &str) -> Option<NativeCall> {
        self.methods.get(name).map(|method| *method)
    }

    fn get_module(&self, _: &str) -> Option<Arc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("oku", Self::parse as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Arc<dyn Module>> {
        HashMap::new()
    }
    
    fn get_classes(&self) -> Vec<Arc<dyn Class>> {
        Vec::new()
    }
}

impl NumModule  {
    pub fn parse(compiler: &mut BramaCompiler, _: Option<Arc<BramaPrimative>>, last_position: usize, total_args: u8) -> NativeCallResult {
        if total_args > 1 {
            return Err(("More than 1 argument passed".to_string(), 0, 0));
        }

        let arg = unsafe { (*compiler.current_scope).stack[last_position - 1].deref() };

        match &*arg {
            BramaPrimative::Number(_) => Ok(unsafe {(*compiler.current_scope).stack[last_position - 1]}),
            BramaPrimative::Text(text) => {
                match (*text).parse() {
                    Ok(num) => Ok(VmObject::native_convert(BramaPrimative::Number(num))),
                    _ => Err(("More than 1 argument passed".to_string(), 0, 0))
                }
            },
            _ => Ok(EMPTY_OBJECT)
        }
    }
}
