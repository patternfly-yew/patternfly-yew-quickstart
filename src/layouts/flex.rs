use crate::{example::ExamplePage, example2, layouts::LayoutItem};
use patternfly_yew::*;
use yew::prelude::*;

pub struct FlexExample {}

impl Component for FlexExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
