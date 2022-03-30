use lazy_static::lazy_static;
use neon::prelude::*;

lazy_static! {
    static ref CORE: libloading::Library = unsafe {
        libloading::Library::new("./resolvers/resolvers-core/target/release/libresolvers_core.so")
            .expect("dlopen")
    };
}

fn get_core<A>(identifier: &[u8]) -> libloading::Symbol<'static, A> {
    unsafe { CORE.get(identifier).expect("symbol lookup") }
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let extern_hello = get_core::<extern "C" fn() -> u32>(b"hello");
    Ok(cx.string(format!("{}", extern_hello())))
}

fn add(mut cx: FunctionContext) -> JsResult<JsString> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let extern_add = get_core::<extern "C" fn(f64, f64) -> f64>(b"add");
    Ok(cx.string(format!("{}", extern_add(x, y))))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    Ok(())
}
