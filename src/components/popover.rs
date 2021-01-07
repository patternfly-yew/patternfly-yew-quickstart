use crate::{example::Example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

pub struct PopoverExample {}

impl Component for PopoverExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <ExamplePage title="Popover">
                    <Example title="Popover Popup" code=include_str!("popover.1.example")>{include!("popover.1.example")}</Example>
                    <Example title="Popover" code=include_str!("popover.2.example")>{include!("popover.2.example")}</Example>
                </ExamplePage>
            </>
        }
    }
}
