use crate::api::register::RegisterRequest;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yew_styles::{button::Button, forms::form_group::FormGroup};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

pub struct Register {
    api: Fetch<RegisterRequest, String>,
    link: ComponentLink<Self>,
    props: RegisterProps,
    username: String,
    password: String,
    password_confirmation: String,
    register_failed: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct RegisterProps {
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    SetApiFetchState(FetchAction<String>),
    SendRegister,
    Nothing,
    SetUsernameField(String),
    SetPasswordField(String),
    SetPasswordConfirmationField(String),
    CloseModal,
    RegisterFailed,
    CloseRegisterFailedModal,
}

impl Component for Register {
    type Message = Msg;
    type Properties = RegisterProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Register {
            api: Default::default(),
            link,
            props,
            username: Default::default(),
            password: Default::default(),
            password_confirmation: Default::default(),
            register_failed: false,
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(_) => {
                        self.link.send_message(Msg::CloseModal);
                    }
                    FetchAction::Failed(_) => {
                        self.link.send_message(Msg::RegisterFailed);
                    }
                    _ => {}
                }
                self.api.apply(fetch_state);

                true
            }
            Msg::SendRegister => {
                self.api.set_req(RegisterRequest::new(
                    self.username.clone(),
                    self.password.clone(),
                ));
                self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                false
            }
            Msg::Nothing => false,
            Msg::SetUsernameField(value) => {
                self.username = value;
                true
            }
            Msg::SetPasswordField(value) => {
                self.password = value;
                true
            }
            Msg::SetPasswordConfirmationField(value) => {
                self.password_confirmation = value;
                true
            }
            Msg::CloseModal => {
                self.props.close_action.emit(());
                true
            }
            Msg::RegisterFailed => {
                self.register_failed = true;
                true
            }
            Msg::CloseRegisterFailedModal => {
                self.register_failed = false;
                self.username = "".to_string();
                self.password = "".to_string();
                self.password_confirmation = "".to_string();
                true
            }
        }
    }

    fn view(&self) -> Html {
        let username_has_error = self.username == "";
        let password_has_error = self.password != self.password_confirmation;
        let username_error_message = if username_has_error {
            "Darf nicht leer sein "
        } else {
            ""
        };
        let password_error_message = if password_has_error {
            "müssen übereinstimmen"
        } else {
            ""
        };
        let register_entry = html! {
        <div class="body-content">
        <FormGroup>
            <FormInput
                input_type=InputType::Text
                input_palette=Palette::Standard
                input_size=Size::Medium
                oninput_signal = self.link.callback(|e: InputData| Msg::SetUsernameField(e.value))
                placeholder="Benutzername"
                underline=false
                error_state=username_has_error
                error_message=username_error_message
            />
            <FormInput
                input_type=InputType::Password
                input_palette=Palette::Standard
                input_size=Size::Medium
                oninput_signal = self.link.callback(|e: InputData| Msg::SetPasswordField(e.value))
                placeholder="Passwort"
                underline=false
            />
            <FormInput
                input_type=InputType::Password
                input_palette=Palette::Standard
                input_size=Size::Medium
                oninput_signal = self.link.callback(|e: InputData| Msg::SetPasswordConfirmationField(e.value))
                placeholder="Passwort bestätigen"
                underline=false
                error_state=password_has_error
                error_message=password_error_message
            />
        </FormGroup>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::SendRegister )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Registrieren"}</Button>
        <Button
             onclick_signal=self.link.callback(move |_| Msg::CloseModal )
             button_palette=Palette::Standard
             button_style=Style::Outline
         >{"Abbrechen"}</Button>
        </div>
        };

        let register_failed_entry = html! {
            <>
            <div>{"Wahrscheinlich existiert schon ein Benutzer mit dem gleichen Namen!"}</div>
            <Button
                onclick_signal=self.link.callback(move |_| Msg::CloseRegisterFailedModal )
                button_palette=Palette::Standard
                button_style=Style::Outline
            >{"Zurück"}
            </Button>
            </>
        };

        if self.register_failed {
            html! {
            <Modal
                header=html!{
                    <b>{"Registrierung fehlgeschlagen"}</b>
                }
                header_palette=Palette::Danger
                body=register_failed_entry
                body_style=Style::Outline
                body_palette=Palette::Danger
                is_open=true
                onclick_signal= self.link.callback(|_|  Msg::Nothing )
                onkeydown_signal= self.link.callback(|_|  Msg::Nothing)
                auto_focus=false
                class_name = "bg"
            />
            }
        } else {
            html! {
            <Modal
                header=html!{
                    <b>{"Registrierung"}</b>
                }
                header_palette=Palette::Link
                body=register_entry
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
