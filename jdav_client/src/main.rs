#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
pub mod api;
pub mod new_entry;

use new_entry::NewEntry;
use yew::InputData;
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;

enum Msg {
    Login,
    SetUserField(String),
    Nothing,
}

enum AppState {
    LoggedOut(String),
    LoggedIn(String),
}

struct Model {
    link: ComponentLink<Self>,
    state: AppState,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            state: AppState::LoggedOut("".to_owned()),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::SetUserField(username) => {
                self.state = AppState::LoggedOut(username);
                false
            }
            Msg::Login => {
                if let AppState::LoggedOut(ref username) = self.state {
                    self.state = AppState::LoggedIn(username.to_owned());
                    true
                } else {
                    false
                }
            }
            Msg::Nothing => false,
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
        <Button
            onclick_signal=self.link.callback(move |_| Msg::Login )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Einloggen"}</Button>
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
        />
        };

        match self.state {
            AppState::LoggedOut(_) => login_modal,
            AppState::LoggedIn(ref username) => {
                html! {
                    <NewEntry username={username}/>
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
