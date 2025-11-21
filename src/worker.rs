use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /* DurableObject */
    pub type DurableObject;

    #[wasm_bindgen(method, getter)]
    pub fn ctx(this: &DurableObject) -> DurableObjectState;

    /* DurableObjectState */
    pub type DurableObjectState;

    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &DurableObjectState) -> DurableObjectStorage;

    /* DurableObjectStorage */
    pub type DurableObjectStorage;
    #[wasm_bindgen(method, getter)]
    pub fn sql(this: &DurableObjectStorage) -> SqlStorage;

    /* DurableObjectStorage */
    pub type SqlStorage;
    #[wasm_bindgen(method)]
    pub fn exec(this: &SqlStorage, query: &str) -> SqlStorageCursor;

    /* SqlStorageCursor */
    pub type SqlStorageCursor;
    #[wasm_bindgen(method)]
    pub fn one(this: &SqlStorageCursor) -> Greeting;

    // (pending generics)
    /* Greeting */
    pub type Greeting;
    #[wasm_bindgen(method, getter)]
    pub fn greeting(this: &Greeting) -> JsString;
}
