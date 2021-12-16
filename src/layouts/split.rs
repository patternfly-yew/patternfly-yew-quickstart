use crate::{example::ExamplePage, example2, layouts::LayoutItem};
use patternfly_yew::*;
use yew::prelude::*;

pub struct SplitExample {}

impl Component for SplitExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example2!("Split" => "split.1.example");
        let example2 = example2!("Split (gutter)" => "split.2.example");
        html! {
            <>
                <ExamplePage title="Split Layout">
                    { example1 }
                    { example2 }
                </ExamplePage>
            </>
        }
    }
}
