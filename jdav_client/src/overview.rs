use yew::Properties;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::new_entry::NewEntry;
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;

#[derive(Clone, Properties, PartialEq)]
pub struct OverviewProps {
    pub username: String,
}
pub enum Msg {
    OpenNewEntry,
    Nothing,
}

enum CurrentAction {
    Nothing,
    NewEntry,
}

pub struct Overview {
    link: ComponentLink<Self>,
    props: OverviewProps,
    current_action: CurrentAction,
}

impl Component for Overview {
    type Message = Msg;
    type Properties = OverviewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Overview {
            link,
            props,
            current_action: CurrentAction::Nothing,
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::OpenNewEntry => {
                self.current_action = CurrentAction::NewEntry;
                true
            }
            Msg::Nothing => false,
        }
    }

    fn view(&self) -> Html {
        let entry = html! {
        <div class="body-content">
        <Button
            onclick_signal=self.link.callback(move |_| Msg::OpenNewEntry )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Neuer Eintrag"}</Button>
        </div>
        };

        let overview_modal = html! {
        <Modal
            header=html!{
                <b>{"Wähle Deine Aktion!"}</b>
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

        match self.current_action {
            CurrentAction::Nothing => overview_modal,
            CurrentAction::NewEntry => {
                html! {
                    <NewEntry username={self.props.username.clone()}/>
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
