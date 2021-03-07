#![recursion_limit = "256"]
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_styles::button::Button;
use yew_styles::forms::form_input::FormInput;
use yew_styles::forms::form_input::InputType;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::modal::Modal;
use yew_styles::styles::Palette;
use yew_styles::styles::Size;
use yew_styles::styles::Style;

struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        let entry = html! {
        <div class="body-content">
        <FormSelect
            select_size=Size::Medium
            onchange_signal = self.link.callback(|_| ())
            options=html!{
                <>
                <option value="laufen">{"Laufen"}</option>
                <option value="radfahren">{"Radfahren"}</option>
                <option value="klettern">{"Klettern"}</option>
                </>
            }
        />
        <FormInput
            input_type=InputType::Text
            input_palette=Palette::Standard
            input_size=Size::Medium
            oninput_signal = self.link.callback(|_| ())
            placeholder="Menge"
            underline=false
        />
        <Button
            onclick_signal=self.link.callback(move |_| ())
            button_palette=Palette::Standard
            button_style=Style::Outline
        >{"Abschicken"}</Button>
        </div>
        };

        html! {
        <Modal
            header=html!{
                <b>{"Leistungen eintragen"}</b>
            }
            header_palette=Palette::Link
            body=entry
            body_style=Style::Outline
            body_palette=Palette::Link
            is_open=true
            onclick_signal= self.link.callback(|_| ())
            onkeydown_signal= self.link.callback(|_| ())
            auto_focus=false
        />
                }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
fn main() {
    yew::start_app::<Model>();
}
