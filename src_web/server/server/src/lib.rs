use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
extern "C"{
    fn alert(msg : &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue >{
    let window = web_sys::window().expect("No Window");
    let document = window.document().expect("No Document");
    let body = document.body().expect("No body");

    let p = document.create_element("p")?;
    p.set_inner_html("<b></b>");

    body.append_child(&p)?;
    Ok(())
}