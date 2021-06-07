use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub enum Msg {
    ShowToast(Toast),
}

pub struct DropdownExample {
    link: ComponentLink<Self>,
}

impl Component for DropdownExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::ShowToast(toast) => {
                ToastDispatcher::new().toast(toast);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example2! ("Dropdown" => "dropdown.1.example");
        let example2 = example2! ("Dropdown (Kebab)" => "dropdown.2.example");
        let example3 = example2! ("Dropdown (User)" => "dropdown.3.example");

        html! {
            <>
                <ExamplePage title="Dropdown">
                    {example1}
                    {example2}
                    {example3}
                </ExamplePage>
            </>
        }
    }
}
