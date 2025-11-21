use js_sys::{JsString, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{Request, Response, Url};
use worker::DurableObject;

mod worker;

/* DO */
#[wasm_bindgen(inline_js = "
import { DurableObject } from 'cloudflare:workers';
export class MyDurableObject extends DurableObject {}
")]
extern "C" {
    #[wasm_bindgen(reexport, extends = DurableObject)]
    pub type MyDurableObject;
}

#[wasm_bindgen(js_namespace = ["MyDurableObject", "prototype"], this, js_name = "sayHello")]
pub fn say_hello(this: &MyDurableObject) -> Result<JsString, JsValue> {
    let result = this
        .ctx()
        .storage()
        .sql()
        .exec("SELECT 'Hello, World!' as greeting")
        .one();
    Ok(result.greeting())
}

/* DO Stub */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type MyDurableObjectStub;

    #[wasm_bindgen(method, js_name = "sayHello")]
    pub async fn say_hello(this: &MyDurableObjectStub) -> JsString;
}
// in lieu of automated bindgen / generics (for now!)...
#[wasm_bindgen]
extern "C" {
    /* DurableObjectNamespace */
    pub type DurableObjectNamespace;

    #[wasm_bindgen(method, js_name = "getByName")]
    pub fn get_by_name(this: &DurableObjectNamespace, name: &str) -> MyDurableObjectStub;
}

/* FETCH */
#[wasm_bindgen(js_name = "fetch", js_namespace = "default")]
pub async fn fetch(request: Request, env: JsValue) -> Result<Response, JsValue> {
    let stub = DurableObjectNamespace::unchecked_from_js(Reflect::get(
        &env,
        &JsValue::from_str("MY_DURABLE_OBJECT"),
    )?)
    .get_by_name(&Url::new(&request.url())?.pathname());

    let greeting = stub.say_hello().await.as_string().unwrap();

    Response::new_with_opt_str(Some(&greeting))
}
