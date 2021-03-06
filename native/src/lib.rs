extern crate reqwest;
extern crate ws;
extern crate serde_json;

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

pub mod meetup;

fn stream(call: Call) -> JsResult<(JsString)> {
    meetup::stream::connect_to_stream();
    let scope = call.scope;
    Ok(JsString::new(scope, "hello").unwrap())
}

register_module!(m, {
    m.export("meetup_stream", stream)
});
