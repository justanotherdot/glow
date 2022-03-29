use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello from rust."))
}

fn add(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(format!("{}", 2 + 2)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    Ok(())
}
