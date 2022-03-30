use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    unsafe {
        let lib = libloading::Library::new(
            "./resolvers/resolvers-core/target/release/libresolvers_core.so",
        )
        .expect("dlopen");
        let extern_hello: libloading::Symbol<unsafe extern "C" fn() -> u32> =
            lib.get(b"hello").expect("symbol lookup");
        Ok(cx.string(format!("{}", extern_hello())))
    }
}

fn add(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(format!("{}", 1 + 2)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    Ok(())
}
