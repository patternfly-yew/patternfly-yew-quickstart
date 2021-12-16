use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct TooltipExample {}

impl Component for TooltipExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
