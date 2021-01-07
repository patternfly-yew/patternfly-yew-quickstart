use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct TooltipExample {}

impl Component for TooltipExample {
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
        let example1 = example2! ("Tooltip Popup" => "tooltip.1.example");
        let example2 = example2! ("Tooltip" => "tooltip.2.example");

        html! {
            <>
                <ExamplePage title="Tooltip">
                    {example1}
                    {example2}
                </ExamplePage>
            </>
        }
    }
}
