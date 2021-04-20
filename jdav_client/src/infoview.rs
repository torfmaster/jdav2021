use enum_iterator::IntoEnumIterator;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{Callback, Properties};
use yew_styles::{
    button::Button,
    layouts::item::{AlignSelf, Item, ItemLayout},
};
use yew_styles::{layouts::container::Container, styles::Style};
use yew_styles::{layouts::container::Direction, modal::Modal};
use yew_styles::{layouts::container::Wrap, styles::Palette};

pub struct InfoView {
    link: ComponentLink<Self>,
    props: InfoProps,
}

#[derive(Clone, Properties, PartialEq)]
pub struct InfoProps {
    pub close_action: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    Nothing,
    CloseModal,
}

impl Component for InfoView {
    type Message = Msg;
    type Properties = InfoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InfoView { link, props }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::Nothing => false,
            Msg::CloseModal => {
                self.props.close_action.emit(());
                false
            }
        }
    }

    fn view(&self) -> Html {
        let entries = shared::Kind::into_enum_iter().map(|item| {
            html! {
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                    {item.to_string()}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                    {item.get_kind_multiplier().to_string()}
                </Item>
                </Container>
            }
        });

        let info_table = html! {
            <Container direction=Direction::Column wrap=Wrap::Nowrap class_name="scrolllist">
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        {"Sportart"}
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        {"Multiplikator"}
                    </Item>
                </Container>
                {entries.collect::<Html>()}
            </Container>
        };
        let entry = html! {
        <div class="body-content">
            {info_table}
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
                <b>{"Info zur Gewichtung der Sportarten"}</b>
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
