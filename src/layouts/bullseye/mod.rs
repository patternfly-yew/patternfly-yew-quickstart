use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct BullseyeExample {}

impl Component for BullseyeExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example! ("Bullseye" => "bullseye.1.example");

        html! {
            <>
                <ExamplePage title="Bullseye Layout">
                    {example1}
                </ExamplePage>
            </>
        }
    }
}
