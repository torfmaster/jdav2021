use shared::UserAuth;
use yew::Properties;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::highscoreview::HighscoreView;
use crate::new_entry::NewEntry;
use yew_styles::button::Button;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Style;

#[derive(Clone, Properties, PartialEq)]
pub struct OverviewProps {
    pub auth: UserAuth,
}
pub enum Msg {
    OpenNewEntry,
    OpenHighScore,
    CloseAction,
    Nothing,
}

enum CurrentAction {
    Nothing,
    NewEntry,
    HighScore,
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
            Msg::CloseAction => {
                self.current_action = CurrentAction::Nothing;
                true
            }
            Msg::OpenHighScore => {
                self.current_action = CurrentAction::HighScore;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let close_action = self.link.callback(|_| Msg::CloseAction);

        let entry = html! {
        <div class="body-content">
        <Button
            onclick_signal=self.link.callback(move |_| Msg::OpenNewEntry )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Neuer Eintrag"}</Button>
        <Button
            onclick_signal=self.link.callback(move |_| Msg::OpenHighScore )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Highscore"}</Button>
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
                    <NewEntry
                      auth={
                          self.props.auth.clone()}
                      close_action={close_action}
                    />
                }
            }
            CurrentAction::HighScore => {
                html! {
                    <HighscoreView
                      auth={self.props.auth.clone()}
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
