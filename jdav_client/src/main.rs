#![recursion_limit = "256"]
use api::BackendRequest;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yewtil::fetch::{Fetch, FetchAction, FetchState};
use yewtil::future::LinkFuture;

mod api;

struct Model {
    api: Fetch<BackendRequest, String>,
    link: ComponentLink<Self>,
    distance: String,
    user: String,
}

pub enum Msg {
    SetApiFetchState(FetchAction<String>),
    PutDistance,
    Nothing,
    SetDistanceField(String),
    SetUserField(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            api: Default::default(),
            link,
            distance: "".to_owned(),
            user: "".to_owned(),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::PutDistance => {
                self.api.set_req(BackendRequest::new(
                    self.distance.clone(),
                    self.user.clone(),
                ));
                self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                false
            }
            Msg::Nothing => false,
            Msg::SetDistanceField(value) => {
                self.distance = value;
                false
            }
            Msg::SetUserField(value) => {
                self.user = value;
                false
            }
        }
    }

    fn view(&self) -> Html {
        let data = match self.api.as_ref().state() {
            FetchState::Fetched(data) => Some(data),
            _ => None,
        };

        let entry = html! {
        <div class="body-content">
        <FormSelect
            select_size=Size::Medium
            onchange_signal = self.link.callback(|_| Msg::Nothing )
            options=html!{
                <>
                <option value="laufen">{"Laufen"}</option>
                <option value="radfahren">{"Radfahren"}</option>
                <option value="klettern">{"Klettern"}</option>
                </>
            }
        />
        <FormInput
            input_type=InputType::Text
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|e: InputData| Msg::SetDistanceField(e.value))
            placeholder="Menge"
            underline=false
        />
        <FormInput
            input_type=InputType::Text
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|e: InputData| Msg::SetUserField(e.value))
            placeholder="Username"
            underline=false
        />
        <Button
            onclick_signal=self.link.callback(move |_| Msg::PutDistance )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Abschicken"}</Button>
        </div>
        };

        html! {
        <Modal
            header=html!{
                <b>{"Leistungen eintragen"}</b>
            }
            header_palette=Palette::Link
            body=entry
            body_style=Style::Outline
            body_palette=Palette::Link
            is_open=true
            onclick_signal= self.link.callback(|_|  Msg::Nothing )
            onkeydown_signal= self.link.callback(|_|  Msg::Nothing)
            auto_focus=false
        />
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
fn main() {
    yew::start_app::<Model>();
}
