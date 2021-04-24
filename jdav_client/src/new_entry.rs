use crate::api::new_entry::KilometerRequest;
use shared::Kind;
use shared::UserAuth;
use yew::{html, ChangeData, Component, ComponentLink, Html, InputData, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yew_styles::{button::Button, forms::form_group::FormGroup};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

pub struct NewEntry {
    api: Fetch<KilometerRequest, String>,
    link: ComponentLink<Self>,
    kind: String,
    props: NewEntryProps,
    parsed_distance: Option<f32>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct NewEntryProps {
    pub auth: UserAuth,
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    SetApiFetchState(FetchAction<String>),
    PutDistance,
    Nothing,
    SetDistanceField(String),
    SetKindField(String),
    CloseConfirmationModal,
}

impl Component for NewEntry {
    type Message = Msg;
    type Properties = NewEntryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NewEntry {
            api: Default::default(),
            link,
            kind: "laufen".to_owned(),
            props,
            parsed_distance: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(_) => {
                        self.link.send_message(Msg::CloseConfirmationModal);
                    }
                    FetchAction::Failed(_) => {}
                    _ => {}
                }
                self.api.apply(fetch_state);
                true
            }
            Msg::PutDistance => {
                if let Some(parsed_distance) = self.parsed_distance {
                    self.api.set_req(KilometerRequest::new(
                        parsed_distance,
                        self.props.auth.clone(),
                        self.kind.clone(),
                    ));
                    self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                    self.link
                        .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                }
                false
            }
            Msg::Nothing => false,
            Msg::SetDistanceField(value) => {
                self.parsed_distance = value.parse::<f32>().ok();
                true
            }
            Msg::SetKindField(value) => {
                self.kind = value;
                true
            }
            Msg::CloseConfirmationModal => {
                self.props.close_action.emit(());
                true
            }
        }
    }

    fn view(&self) -> Html {
        let select_callback = |e: ChangeData| match e {
            ChangeData::Value(_) => Msg::Nothing,
            ChangeData::Select(v) => Msg::SetKindField(v.value()),
            ChangeData::Files(_) => Msg::Nothing,
        };

        let error = if self.parsed_distance.is_none() {
            "Muss eine gültige Zahl sein, z.B. 1.0 (mit Punkt!)"
        } else {
            ""
        };

        let entry = html! {
        <div class="body-content">
        <FormGroup>
            <FormSelect
                select_size=Size::Medium
                onchange_signal = self.link.callback(select_callback)
                options=html!{
                    <>
                    <option value={Kind::Running.get_path()}>{"Laufen"}</option>
                    <option value={Kind::Biking.get_path()}>{"Radfahren"}</option>
                    <option value={Kind::Climbing.get_path()}>{"Klettern"}</option>
                    <option value={Kind::Hiking.get_path()}>{"Wandern"}</option>
                    <option value={Kind::Swimming.get_path()}>{"Schwimmen"}</option>
                    <option value={Kind::Skating.get_path()}>{"Skaten"}</option>
                    <option value={Kind::NordicWalking.get_path()}>{"Nordic Walking"}</option>
                    </>
                }
            />
            <FormInput
                input_palette=Palette::Standard
                input_size=Size::Medium
                error_state=self.parsed_distance.is_none()
                error_message=error
                oninput_signal = self.link.callback(|e: InputData| Msg::SetDistanceField(e.value))
                placeholder="Distanz (in km)"
                underline=false
            />
        </FormGroup>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::PutDistance )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Abschicken"}</Button>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::CloseConfirmationModal )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Abbrechen"}</Button>
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
            class_name = "bg"
        />
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
