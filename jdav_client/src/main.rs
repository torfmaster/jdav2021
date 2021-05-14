#![recursion_limit = "512"]

use api::login::LoginRequest;
use overview::OverviewRoute;
use shared::UserAuth;
use web_sys::MouseEvent;
use yew::InputData;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router::prelude::*;
use yew_router::router::Router;
use yew_router::Switch;
use yew_styles::button::Button;
use yew_styles::forms::form_group::FormGroup;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;
use yew_styles::text::{Text, TextType};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use crate::overview::Overview;
use crate::register::Register;

pub mod api;
pub mod edit_entry;
pub mod entriesview;
pub mod highscoreview;
mod infoview;
pub mod new_entry;
pub mod overview;
pub mod register;

enum Msg {
    StartLogin,
    SetUserField(String),
    SetPasswordField(String),
    Nothing,
    SetApiFetchState(FetchAction<String>),
    FinalizeLogin,
    Navigate(MainRoute),
}

enum AppState {
    LoggedOut(UserAuth),
    LoggedIn(UserAuth),
}

#[derive(Switch, Clone, Debug)]
pub enum MainRoute {
    #[to = "/register"]
    Registration,
    #[to = "/overview{*:inner}"]
    Overview(OverviewRoute),
    #[to = "/"]
    Login,
    #[to = "/login_failed"]
    LoginFailed,
}

struct Model {
    link: ComponentLink<Self>,
    api: Fetch<LoginRequest, String>,
    state: AppState,
    router: RouteAgentDispatcher<()>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();

        Model {
            api: Default::default(),
            link,
            state: AppState::LoggedOut(Default::default()),
            router,
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
            Msg::Nothing => false,
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(_) => {
                        self.link.send_message(Msg::FinalizeLogin);
                    }
                    FetchAction::Failed(_) => {
                        self.link
                            .send_message(Msg::Navigate(MainRoute::LoginFailed));
                    }
                    _ => {}
                }
                self.api.apply(fetch_state);

                true
            }
            Msg::FinalizeLogin => {
                if let AppState::LoggedOut(ref username) = self.state {
                    self.state = AppState::LoggedIn(username.to_owned());
                    self.link
                        .send_message(Msg::Navigate(MainRoute::Overview(OverviewRoute::Overview)));
                    true
                } else {
                    false
                }
            }
            Msg::Navigate(location) => {
                self.router.send(RouteRequest::ChangeRoute(location.into()));
                true
            }
        }
    }

    fn view(&self) -> Html {
        let goto_overview = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Navigate(MainRoute::Overview(OverviewRoute::Overview))
        });

        let goto_registration = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Navigate(MainRoute::Registration)
        });

        let goto_main_page = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Navigate(MainRoute::Login)
        });

        let goto_login_failed = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Navigate(MainRoute::Login)
        });

        let navigate_to = self
            .link
            .callback(move |route: MainRoute| Msg::Navigate(route.clone()));

        let entry = html! {
        <div class="body-content">
        <FormGroup>
        <FormInput
            input_type=InputType::Text
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|e: InputData| Msg::SetUserField(e.value))
            placeholder="Benutzername"
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
        </FormGroup>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::StartLogin )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Einloggen"}</Button>
        <Button
            onclick_signal=goto_registration
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Registrieren"}</Button>
        <FormGroup>
        <a href="https://www.alpenverein-regensburg.de/index.php/impressum">
            <Text
                text_type=TextType::Plain
                text_size=Size::Small
                plain_text="Impressum"
                html_text=None
            />
        </a>
        </FormGroup>
        </div>
        };

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
            class_name = "bg"
        />
        };

        let login_failed_body = html! {
            <Button
                onclick_signal=goto_main_page
                button_palette=Palette::Standard
                button_style=Style::Outline
            >{"Schade..."}
            </Button>
        };
        let close_action = self
            .link
            .callback(move |_| Msg::Navigate(OverviewRoute::Overview.into()));

        match &self.state {
            AppState::LoggedOut(_) => {
                html! {
                    <Router<MainRoute>
                        render=Router::render(move |switch: MainRoute| {
                            match switch {
                                MainRoute::Registration => {
                                    html! {
                                        <Register close_action={close_action.clone()}/>
                                    }
                                }
                                _ => login_modal.clone(),
                            }
                        })
                        redirect = Router::redirect(|_: Route| {
                            MainRoute::Overview(OverviewRoute::Overview)
                        })
                    />
                }
            }
            AppState::LoggedIn(user_auth) => {
                let user_auth = user_auth.clone();
                html! {
                    <Router<MainRoute>
                        render=Router::render(move |switch: MainRoute| {
                            match switch {
                                MainRoute::Overview(overview_route) => {
                                    let overview_route = overview_route.clone();
                                    html! {
                                        <Overview auth={user_auth.clone()} navigate={navigate_to.clone()} route={overview_route}/>
                                    }
                                }
                                _ => login_modal.clone(),
                            }
                        })
                        redirect = Router::redirect(|_: Route| {
                            MainRoute::Overview(OverviewRoute::Overview)
                        })
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
