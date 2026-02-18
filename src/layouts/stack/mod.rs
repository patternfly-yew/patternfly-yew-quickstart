use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct StackExample {}

impl Component for StackExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example!("Stack" => "stack.1.example");
        let example2 = example!("Stack (gutter)" => "stack.2.example");
        html! {
            <>
                <ExamplePage title="Stack Layout">{ example1 }{ example2 }</ExamplePage>
            </>
        }
    }
}
