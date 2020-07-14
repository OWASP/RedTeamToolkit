use yew::{
      Component
    , ComponentLink
    , html
    , Html
    , ShouldRender
};

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
        let should_render : bool = false;
        // NOTE: change should_render to true if any changes are made that will update the visual dom
        should_render && true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let should_render : bool = false;
        // NOTE: change should_render to true if any changes are made that will update the visual dom
        should_render && true
    }
    fn view(&self) -> Html {
        html! {
            <>
                <header class="full--width">
                    {"test"}
                </header>
            </>
        }
    }
}