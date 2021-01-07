use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct FlexExample {}

impl Component for FlexExample {
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
        let style = "border: 1px solid;";

        let example1 = example2! ("Flex" => "flex.1.example");
        let example2 = example2! ("Flex (nested and grow)" => "flex.2.example");
        let example3 = example2! ("Flex (column on lg)" => "flex.3.example");

        html! {
            <>
                <ExamplePage title="Flex Layout">
                    {example1}
                    {example2}
                    {example3}
                </ExamplePage>
            </>
        }
    }
}
