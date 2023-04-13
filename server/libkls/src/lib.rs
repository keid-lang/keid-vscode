use keid::compiler::{llvm::Context, ClassInfoStorage, Compiler, SignatureCompiler};
use wasm_bindgen::prelude::*;

pub struct KlsContext {
    sigs: SignatureCompiler,
    error: Option<String>,
}

#[wasm_bindgen]
pub fn libkls_create_context() -> *mut KlsContext {
    console_error_panic_hook::set_once();
    
    let sigs = SignatureCompiler::new();

    Box::into_raw(Box::new(KlsContext {
        sigs,
        error: None,
    }))
}

#[wasm_bindgen]
pub fn libkls_get_last_error(ctx: *mut KlsContext) -> Option<String> {
    let ctx = unsafe { Box::from_raw(ctx) };
    let error = ctx.error.clone();
    Box::into_raw(ctx);
    error
}

#[wasm_bindgen]
pub fn libkls_include_file(ctx: *mut KlsContext, path: &str, contents: &str) -> bool {
    let mut ctx = unsafe { Box::from_raw(ctx) };

    let new_file = match keid::parser::parse(&path, &contents) {
        Ok(file) => file,
        Err(err) => {
            ctx.error = Some(format!("Error in {}:\n{:#?}", path, err));
            Box::into_raw(ctx);
            return true;
        }
    };

    for mut file in &mut ctx.sigs.source_files {
        if file.source_path == path {
            file.root_statements = new_file.root_statements;
            Box::into_raw(ctx);
            return false;
        }
    }

    ctx.sigs.add_file(new_file);
    Box::into_raw(ctx);

    false
}

#[wasm_bindgen]
pub fn libkls_compile(ctx: *mut KlsContext) -> Option<js_sys::Array> {
    let ctx = unsafe { Box::from_raw(ctx) };

    let mut context = Context::new();
    let resources = ctx.sigs.compile(&mut context);

    let class_info = ClassInfoStorage::new(&mut context);
    let mut compiler = Compiler::new(class_info, context);
    if compiler.compile(resources) {
        let errors = compiler.get_errors();
        let arr = js_sys::Array::new_with_length(errors.len() as _);
        for i in 0..arr.length() {
            let error = &errors[i as usize];
            let obj = js_sys::Object::new();
            #[allow(unused_unsafe)]
            unsafe {
                js_sys::Reflect::set(&obj, &"file".into(), &JsValue::from_str(&error.0)).unwrap();
                js_sys::Reflect::set(&obj, &"message".into(), &error.1.message.clone().into()).unwrap();

                let range = js_sys::Object::new();
                js_sys::Reflect::set(&range, &"start".into(), &error.1.loc.start.into()).unwrap();
                js_sys::Reflect::set(&range, &"end".into(), &error.1.loc.end.into()).unwrap();

                js_sys::Reflect::set(&obj, &"range".into(), &range.into()).unwrap();
            }
            arr.set(i, obj.into());
        }

        Box::into_raw(ctx);
        return Some(arr);
    }

    Box::into_raw(ctx);
    None
}

#[wasm_bindgen]
pub fn libkls_dispose_context(ctx: *mut KlsContext) {
    unsafe {
        std::mem::drop(Box::from_raw(ctx));
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_parse() {}
}
