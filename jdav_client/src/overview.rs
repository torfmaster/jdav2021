use shared::UserAuth;
use yew::Properties;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::entriesview::EntriesView;
use crate::highscoreview::HighscoreView;
use crate::infoview::InfoView;
use crate::new_entry::NewEntry;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Style;
use yew_styles::{card::Card, styles::Size};

use yew_styles::layouts::container::{Container, Direction, Wrap};

#[derive(Clone, Properties, PartialEq)]
pub struct OverviewProps {
    pub auth: UserAuth,
}
pub enum Msg {
    OpenNewEntry,
    OpenHighScore,
    OpenEntriesView,
    OpenInfoView,
    CloseAction,
    Nothing,
}

enum CurrentAction {
    Nothing,
    NewEntry,
    HighScore,
    EntriesView,
    InfoView,
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
            Msg::OpenEntriesView => {
                self.current_action = CurrentAction::EntriesView;
                true
            }
            Msg::OpenInfoView => {
                self.current_action = CurrentAction::InfoView;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let close_action = self.link.callback(|_| Msg::CloseAction);

        let entry = html! {
        <Container direction=Direction::Column wrap=Wrap::Nowrap class_name="overview">
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Neuer Eintrag"}</h1>}
            onclick_signal=self.link.callback(move |_| Msg::OpenNewEntry )
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Meine Einträge"}</h1>}
            onclick_signal=self.link.callback(move |_| Msg::OpenEntriesView )
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Info"}</h1>}
            onclick_signal=self.link.callback(move |_| Msg::OpenInfoView )
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Highscore"}</h1>}
            onclick_signal=self.link.callback(move |_| Msg::OpenHighScore )
        />
        </Container>
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
            class_name = "bg"
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
            CurrentAction::EntriesView => {
                html! {
                    <EntriesView
                      auth={self.props.auth.clone()}
                      close_action={close_action}
                    />
                }
            }
            CurrentAction::InfoView => {
                html! {
                    <InfoView
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
