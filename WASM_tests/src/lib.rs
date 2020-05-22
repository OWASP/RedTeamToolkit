use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn say_hi(name: &str) -> Result<(), JsValue>{
    let ws = WebSocket::new("wss://echo.websocket.org")?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();
    let onmessage = Closure::wrap(Box::new(move | m: MessageEvent| {
        if let abuf = m.data().dyn_into::<js_sys::ArrayBuffer>() {
            alert(&format!("Message: {:?}", abuf));
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    let cloned_ws = ws.clone();
    let onopen = Closure::wrap(Box::new(move |js_name : JsValue| {
        let res = cloned_ws.send_with_str("hi");
    })as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();
    // alert(&format!("Hello, {}", name));
    return Ok(());
}