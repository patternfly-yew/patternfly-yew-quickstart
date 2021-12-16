use crate::{example::ExamplePage, example2, layouts::LayoutItem};
use patternfly_yew::*;
use yew::prelude::*;

pub struct StackExample {}

impl Component for StackExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example2!("Stack" => "stack.1.example");
        let example2 = example2!("Stack (gutter)" => "stack.2.example");
        html! {
            <>
                <ExamplePage title="Stack Layout">
                    { example1 }
                    { example2 }
                </ExamplePage>
            </>
        }
    }
}
