use shared::UserAuth;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{Callback, Properties};

use crate::highscoreview::HighscoreView;
use crate::infoview::InfoView;
use crate::new_entry::NewEntry;
use crate::{entriesview::EntriesView, MainRoute};
use yew_router::Switch;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Style;
use yew_styles::{card::Card, styles::Size};

#[derive(Switch, Clone, Debug, PartialEq)]
pub enum OverviewRoute {
    #[to = "/highscore"]
    HighScore,
    #[to = "/new_entry"]
    NewEntry,
    #[to = "/edit_entry"]
    EditEntries,
    #[to = "/info"]
    Info,
    #[to = "/"]
    Overview,
}

impl Into<MainRoute> for OverviewRoute {
    fn into(self) -> MainRoute {
        MainRoute::Overview(self)
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct OverviewProps {
    pub auth: UserAuth,
    pub route: OverviewRoute,
    pub navigate: Callback<MainRoute>,
}
pub enum Msg {
    Navigate(MainRoute),
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
            Msg::Navigate(location) => {
                self.props.navigate.emit(location.into());
                false
            }
            Msg::Nothing => false,
        }
    }

    fn view(&self) -> Html {
        let navigate_to =
            |location: MainRoute| self.link.callback(move |_| Msg::Navigate(location.clone()));

        let close_action = self
            .link
            .callback(move |_| Msg::Navigate(OverviewRoute::Overview.into()));

        let entry = html! {
        <div class="scrolllist">
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Neuer Eintrag"}</h1>}
            onclick_signal={navigate_to(OverviewRoute::NewEntry.into())}
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Meine Einträge"}</h1>}
            onclick_signal={navigate_to(OverviewRoute::EditEntries.into())}
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Info"}</h1>}
            onclick_signal={navigate_to(OverviewRoute::Info.into())}
        />
        <Card
            card_size=Size::Small
            card_palette=Palette::Success
            card_style=Style::Outline
            body=html!{<h1>{"Highscore"}</h1>}
            onclick_signal={navigate_to(OverviewRoute::HighScore.into())}
        />

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
            class_name = "bg"
        />
        };

        match self.props.route {
            OverviewRoute::Overview => overview_modal,
            OverviewRoute::NewEntry => {
                html! {
                    <NewEntry
                      auth={
                          self.props.auth.clone()}
                      close_action={close_action}
                    />
                }
            }
            OverviewRoute::HighScore => {
                html! {
                    <HighscoreView
                      auth={self.props.auth.clone()}
                      close_action={close_action}
                    />
                }
            }
            OverviewRoute::EditEntries => {
                html! {
                    <EntriesView
                      auth={self.props.auth.clone()}
                      close_action={close_action}
                    />
                }
            }
            OverviewRoute::Info => {
                html! {
                    <InfoView
                        close_action={close_action}
                    />
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let has_changed = self.props != props;
        self.props = props;
        has_changed
    }
}
