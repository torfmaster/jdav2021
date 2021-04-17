use chrono::prelude::*;
use shared::{Entries, UserAuth};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::{
    button::Button,
    card::Card,
    layouts::item::{Item, ItemLayout},
    styles::Size,
};
use yew_styles::{layouts::container::Container, styles::Style};
use yew_styles::{layouts::container::Direction, modal::Modal};
use yew_styles::{layouts::container::Wrap, styles::Palette};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use crate::api::get_entries::EntriesRequest;
use crate::edit_entry::EditEntry;

pub struct EntriesView {
    link: ComponentLink<Self>,
    api: Fetch<EntriesRequest, Entries>,
    props: EntriesProps,
    pub content: Entries,
    edit_entry_idx: usize,
    current_action: CurrentAction,
}

#[derive(Clone, Properties, PartialEq)]
pub struct EntriesProps {
    pub auth: UserAuth,
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    SetApiFetchState(FetchAction<Entries>),
    OpenEditEntry(usize),
    CloseEditEntry,
    Nothing,
    CloseModal,
    InitList,
}

enum CurrentAction {
    Nothing,
    EditEntry,
}

impl Component for EntriesView {
    type Message = Msg;
    type Properties = EntriesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model = EntriesView {
            link,
            props,
            api: Default::default(),
            content: Default::default(),
            edit_entry_idx: 0,
            current_action: CurrentAction::Nothing,
        };
        model.link.send_message(Msg::InitList);
        model
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::Nothing => false,
            Msg::CloseModal => {
                self.props.close_action.emit(());
                false
            }
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(ref response) => self.content = response.clone(),
                    FetchAction::Failed(_) => {}
                    _ => {}
                }
                self.api.apply(fetch_state);
                true
            }
            Msg::OpenEditEntry(entry) => {
                self.current_action = CurrentAction::EditEntry;
                self.edit_entry_idx = entry;
                true
            }
            Msg::CloseEditEntry => {
                self.current_action = CurrentAction::Nothing;
                self.link.send_message(Msg::InitList);
                true
            }
            Msg::InitList => {
                self.api
                    .set_req(EntriesRequest::new(self.props.auth.clone()));
                self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                false
            }
        }
    }

    fn view(&self) -> Html {
        let entries = self.content.list.iter().enumerate().rev().map(|(idx, item)| {
            let card_pal = match item.kind {
                shared::Kind::Biking => Palette::Primary,
                shared::Kind::Climbing => Palette::Success,
                shared::Kind::Running => Palette::Info,
                shared::Kind::Hiking => Palette::Clean,
                shared::Kind::Skating => Palette::Danger,
                shared::Kind::Swimming => Palette::Success
            };
            html! {
                <Item layouts=vec!(ItemLayout::ItM(3))>
                    <Card
                        card_size=Size::Small
                        card_palette=card_pal
                        card_style=Style::Outline
                        header=html!{<b>{item.kind.clone()}</b>}
                        body=html!{
                            <div>
                                {item.timestamp.with_timezone(&FixedOffset::east(2*3600)).format("Vom: %d.%m.%y, %H:%M").to_string()}
                                <br/>
                                {format!("Distanz: {}", item.kilometers.clone())}
                                <br/>
                                <Button
                                    onclick_signal=self.link.callback(move |_| Msg::OpenEditEntry(idx) )
                                    button_palette=Palette::Standard
                                    button_style=Style::Outline
                                >
                                {"Bearbeiten"}
                                </Button>
                            </div>
                        }
                    />
               </Item>
            }
        });

        let entries_table = html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="scrolllist">
                {entries.collect::<Html>()}
            </Container>
        };
        let entry = html! {
        <div class="body-content">
            {entries_table}
            <Button
                onclick_signal=self.link.callback(move |_| Msg::CloseModal )
                button_palette=Palette::Standard
                button_style=Style::Outline
            >
            {"Schließen"}
            </Button>
        </div>
        };

        let close_action = self.link.callback(|_| Msg::CloseEditEntry);

        match self.current_action {
            CurrentAction::Nothing => {
                html! {
                    <Modal
                    header=html!{
                        <b>{"Meine Einträge"}</b>
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
            CurrentAction::EditEntry => {
                let e = self.content.list.get(self.edit_entry_idx).clone().unwrap();
                html! {
                    <EditEntry
                        auth = {self.props.auth.clone()}
                        entry = {e}
                        close_action={close_action}
                    />
                }
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
