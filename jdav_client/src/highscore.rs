use crate::api::BackendRequest;
use yew::{
    html, services::ConsoleService, ChangeData, Component, ComponentLink, Html, InputData,
    ShouldRender,
};
use yew::{Callback, Properties};
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;

pub struct HighScore {
    link: ComponentLink<Self>,
    props: HighscoreProps,
}

#[derive(Clone, Properties, PartialEq)]
pub struct HighscoreProps {
    pub username: String,
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    Nothing,
    CloseModal,
}

impl Component for HighScore {
    type Message = Msg;
    type Properties = HighscoreProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HighScore { link, props }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        ConsoleService::info(&format!("Update: {:?}", message));
        match message {
            Msg::Nothing => false,
            Msg::CloseModal => {
                self.props.close_action.emit(());
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
