use shared::{Highscore, UserAuth};
use yew::{html, services::ConsoleService, Component, ComponentLink, Html, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::button::Button;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Style;
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use crate::api::highscore::HighscoreRequest;

pub struct HighscoreView {
    link: ComponentLink<Self>,
    api: Fetch<HighscoreRequest, Highscore>,
    props: HighscoreProps,
    pub content: String,
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
        ConsoleService::info(&format!("Update: {:?}", message));
        match message {
            Msg::Nothing => false,
            Msg::CloseModal => {
                self.props.close_action.emit(());
                false
            }
            Msg::SetApiFetchState(fetch_state) => {
                match fetch_state {
                    FetchAction::Fetched(ref response) => self.content = format!("{:?}", response),
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
        let entry = html! {
        <div class="body-content">
        <Button
            onclick_signal=self.link.callback(move |_| Msg::CloseModal )
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Abbrechen"}</Button>
        {self.content.clone()}
        </div>
        };

        html! {
        <Modal
            header=html!{
                <b>{"Hier könnten Highscores stehen"}</b>
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
