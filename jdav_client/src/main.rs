#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
pub mod api;
pub mod new_entry;

use new_entry::NewEntry;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, message: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <NewEntry/>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
fn main() {
    yew::start_app::<Model>();
}
