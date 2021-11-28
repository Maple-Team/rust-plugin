#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, ContextlessResult, Env, JsNumber, JsObject, JsString, Result, Task};
use uuid::Uuid;

struct AsyncTask(u32);

impl Task for AsyncTask {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_millis(self.0 as u64));
        Ok(self.0 * 2)
    }

    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("sync", sync_fn)?;

    exports.create_named_method("sleep", sleep)?;

    exports.create_named_method("uuid", uuid)?;
    exports.create_named_method("uuidv4", uuidv4)?;
    Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

    ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let task = AsyncTask(argument);
    let async_task = ctx.env.spawn(task)?;
    Ok(async_task.promise_object())
}

#[contextless_function]
fn uuid(env: Env) -> ContextlessResult<JsString> {
    let id = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    env.create_string(id.to_string().as_str()).map(Some)
}
#[contextless_function]
fn uuidv4(env: Env) -> ContextlessResult<JsString> {
    let id = Uuid::new_v4();
    env.create_string(id.to_string().as_str()).map(Some)
}
