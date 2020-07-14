use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use web_sys::*;
use yew::prelude::*;
mod components;
use components::header::Header as Header;
#[wasm_bindgen]
extern "C"{
    fn alert(msg : &str);
}
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue >{
    let window = web_sys::window().expect("No Window");
    let document = window.document().expect("No Document");
    let body = document.body().expect("No body");
    App::<Header>::new().mount_to_body();
    let jheader = document.create_element("header")?;
    jheader.set_inner_html("<h1>RedTeam Toolkit</h1>");
    let p = document.create_element("p")?;
    p.set_inner_html("<b>this is a test</b>");
    body.append_child(&jheader)?;
    body.append_child(&p)?;
    Ok(())
}