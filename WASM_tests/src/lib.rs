use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket, BinaryType};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn say_hi(name: &str) -> Result<(), JsValue>{
    let ws = WebSocket::new("wss://echo.websocket.org")?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    alert(&format!("Hello, {}", name));
    return Ok(());
}