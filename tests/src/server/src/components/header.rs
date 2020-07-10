// use yew::{
//       Component
//     , ComponentLink
//     , html
//     , Html
//     , ShouldRender
// };
use yew::prelude::*;

pub struct Header {
    link : ComponentLink<Self>,
}

pub enum Event {
    Click,
}

impl Component for Header {
    type Message = Event;
    type Properties = ();

    fn create(_ : Self::Properties, link : ComponentLink<Self>) -> Self {
        Header { link }
    }
    fn update(&mut self, ev : Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    {"test"}
                </header>
            </>
        }
    }
}