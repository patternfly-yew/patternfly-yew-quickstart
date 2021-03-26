use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct SwitchExample {}

impl Component for SwitchExample {
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
        let example1 = example2! ("Switch" => "switch.1.example");
        let example2 = example2! ("Switch (label)" => "switch.2.example");
        let example3 = example2! ("Switch (disabled)" => "switch.3.example");

        html! {
            <>
                <ExamplePage title="Badge">
                    {example1}
                    {example2}
                    {example3}
                </ExamplePage>
            </>
        }
    }
}
