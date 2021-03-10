use crate::api::BackendRequest;
use yew::{
    html, services::ConsoleService, ChangeData, Component, ComponentLink, Html, InputData,
    ShouldRender,
};
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

pub struct NewEntry {
    api: Fetch<BackendRequest, String>,
    link: ComponentLink<Self>,
    distance: String,
    user: String,
    kind: String,
    add_entry_modal_open: bool,
}

#[derive(Debug)]
pub enum Msg {
    SetApiFetchState(FetchAction<String>),
    PutDistance,
    Nothing,
    SetDistanceField(String),
    SetUserField(String),
    SetKindField(String),
    CloseConfirmationModal,
}

impl Component for NewEntry {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NewEntry {
            api: Default::default(),
            link,
            distance: "".to_owned(),
            user: "".to_owned(),
            kind: "laufen".to_owned(),
            add_entry_modal_open: true,
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        ConsoleService::info(&format!("Update: {:?}", message));
        match message {
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(_) => {
                        self.add_entry_modal_open = false;
                    }
                    FetchAction::Failed(_) => {}
                    _ => {}
                }
                self.api.apply(fetch_state);
                true
            }
            Msg::PutDistance => {
                self.api.set_req(BackendRequest::new(
                    self.distance.clone(),
                    self.user.clone(),
                    self.kind.clone(),
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
            Msg::SetKindField(value) => {
                self.kind = value;
                false
            }
            Msg::CloseConfirmationModal => {
                self.add_entry_modal_open = false;
                false
            }
        }
    }

    fn view(&self) -> Html {
        let select_callback = |e: ChangeData| match e {
            ChangeData::Value(_) => Msg::Nothing,
            ChangeData::Select(v) => Msg::SetKindField(v.value()),
            ChangeData::Files(_) => Msg::Nothing,
        };

        let entry = html! {
        <div class="body-content">
        <FormSelect
            select_size=Size::Medium
            onchange_signal = self.link.callback(select_callback)
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
            is_open=self.add_entry_modal_open
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
