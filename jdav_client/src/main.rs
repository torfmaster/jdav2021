#![recursion_limit = "512"]

use api::login::LoginRequest;
use shared::UserAuth;
use yew::InputData;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use crate::overview::Overview;
use crate::register::Register;

pub mod api;
pub mod highscore;
pub mod new_entry;
pub mod overview;
pub mod register;

enum Msg {
    StartLogin,
    Register,
    SetUserField(String),
    SetPasswordField(String),
    CloseAction,
    Nothing,
    SetApiFetchState(FetchAction<String>),
    FinalizeLogin,
}

enum AppState {
    LoggedOut(UserAuth),
    LoggedIn(UserAuth),
    Register,
}

struct Model {
    link: ComponentLink<Self>,
    api: Fetch<LoginRequest, String>,
    state: AppState,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            api: Default::default(),
            link,
            state: AppState::LoggedOut(Default::default()),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::SetUserField(username) => {
                if let AppState::LoggedOut(ref user_auth) = self.state {
                    self.state = AppState::LoggedOut(UserAuth {
                        name: username,
                        ..user_auth.clone()
                    });
                }
                false
            }
            Msg::SetPasswordField(password) => {
                if let AppState::LoggedOut(ref user_auth) = self.state {
                    self.state = AppState::LoggedOut(UserAuth {
                        pass: password,
                        ..user_auth.clone()
                    });
                }
                false
            }
            Msg::StartLogin => {
                if let AppState::LoggedOut(ref user_auth) = self.state {
                    self.api.set_req(LoginRequest {
                        payload: user_auth.clone(),
                    });
                    self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                    self.link
                        .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                    false
                } else {
                    false
                }
            }
            Msg::Register => {
                self.state = AppState::Register;
                true
            }
            Msg::Nothing => false,
            Msg::CloseAction => {
                self.state = AppState::LoggedOut(Default::default());
                true
            }
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(_) => {
                        self.link.send_message(Msg::FinalizeLogin);
                    }
                    FetchAction::Failed(_) => {}
                    _ => {}
                }
                self.api.apply(fetch_state);

                true
            }
            Msg::FinalizeLogin => {
                if let AppState::LoggedOut(ref username) = self.state {
                    self.state = AppState::LoggedIn(username.to_owned());
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self) -> Html {
        let entry = html! {
        <div class="body-content">
        <FormInput
            input_type=InputType::Text
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|e: InputData| Msg::SetUserField(e.value))
            placeholder="Username"
            underline=false
        />
        <FormInput
            input_type=InputType::Password
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|e: InputData| Msg::SetPasswordField(e.value))
            placeholder="Passwort"
            underline=false
        />
        <Button
            onclick_signal=self.link.callback(move |_| Msg::StartLogin )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Einloggen"}</Button>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::Register )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Registrieren"}</Button>
        </div>
        };

        let close_action = self.link.callback(|_| Msg::CloseAction);

        let login_modal = html! {
        <Modal
            header=html!{
                <b>{"Bitte einloggen"}</b>
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
        };

        match self.state {
            AppState::LoggedOut(_) => login_modal,
            AppState::LoggedIn(ref user_auth) => {
                html! {
                    <Overview auth={user_auth.clone()}/>
                }
            }
            AppState::Register => {
                html! {
                    <Register
                        close_action={close_action}
                    />
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
fn main() {
    yew::start_app::<Model>();
}
