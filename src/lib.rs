use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function js_sleep(millis) { return new Promise((resolve) => setTimeout(resolve, millis)); }"
)]
extern "C" {
    pub fn js_sleep(millis: i32) -> js_sys::Promise;
}

pub async fn sleep(millis: i32) {
    let promise = js_sleep(millis);
    let js_fut = wasm_bindgen_futures::JsFuture::from(promise);
    js_fut.await.expect("Failed to await JS future");
}
