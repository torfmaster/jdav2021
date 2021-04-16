use shared::{Highscore, UserAuth};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::{
    button::Button,
    layouts::item::{AlignSelf, Item, ItemLayout},
};
use yew_styles::{layouts::container::Container, styles::Style};
use yew_styles::{layouts::container::Direction, modal::Modal};
use yew_styles::{layouts::container::Wrap, styles::Palette};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use crate::api::highscore::HighscoreRequest;

pub struct HighscoreView {
    link: ComponentLink<Self>,
    api: Fetch<HighscoreRequest, Highscore>,
    props: HighscoreProps,
    pub content: Highscore,
}

#[derive(Clone, Properties, PartialEq)]
pub struct HighscoreProps {
    pub auth: UserAuth,
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    SetApiFetchState(FetchAction<Highscore>),
    Nothing,
    CloseModal,
    InitList,
}

impl Component for HighscoreView {
    type Message = Msg;
    type Properties = HighscoreProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model = HighscoreView {
            link,
            props,
            api: Default::default(),
            content: Default::default(),
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
            Msg::InitList => {
                self.api
                    .set_req(HighscoreRequest::new(self.props.auth.clone()));
                self.link.send_future(self.api.fetch(Msg::SetApiFetchState));
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                false
            }
        }
    }

    fn view(&self) -> Html {
        let entries = self.content.list.iter().enumerate().map(|(pos, item)| {
            html! {
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                    {pos+1}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                    {item.user.clone()}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                    {item.points.clone()}
                </Item>
                </Container>
            }
        });

        let highscore_table = html! {
            <Container direction=Direction::Column wrap=Wrap::Nowrap class_name="scrolllist">
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        {"Platz"}
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        {"Name"}
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        {"Punktzahl"}
                    </Item>
                </Container>
                {entries.collect::<Html>()}
            </Container>
        };
        let entry = html! {
        <div class="body-content">
            {highscore_table}
            <Button
                onclick_signal=self.link.callback(move |_| Msg::CloseModal )
                button_palette=Palette::Standard
                button_style=Style::Outline
            >
            {"Schließen"}
            </Button>
        </div>
        };

        html! {
        <Modal
            header=html!{
                <b>{"Bestenliste"}</b>
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
