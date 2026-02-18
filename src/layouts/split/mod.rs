use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct SplitExample {}

impl Component for SplitExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example!("Split" => "split.1.example");
        let example2 = example!("Split (gutter)" => "split.2.example");
        html! {
            <>
                <ExamplePage title="Split Layout">{ example1 }{ example2 }</ExamplePage>
            </>
        }
    }
}
