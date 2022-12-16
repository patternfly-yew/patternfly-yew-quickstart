use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub enum Msg {
    ShowToast(Toast),
}

pub struct DropdownExample {}

impl Component for DropdownExample {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowToast(toast) => {
                if let Some((toaster, _)) = ctx.link().context::<Toaster>(Callback::default()) {
                    toaster.toast(toast);
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
