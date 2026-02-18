use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct FlexExample {}

impl Component for FlexExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example! ("Flex" => "flex.1.example");
        let example2 = example! ("Flex (nested and grow)" => "flex.2.example");
        let example3 = example! ("Flex (column on lg)" => "flex.3.example");

        html! {
            <>
                <ExamplePage title="Flex Layout">{ example1 }{ example2 }{ example3 }</ExamplePage>
            </>
        }
    }
}
