use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct BullseyeExample {}

impl Component for BullseyeExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example2! ("Bullseye" => "bullseye.1.example");

        html! {
            <>
                <ExamplePage title="Bullseye Layout">
                    {example1}
                </ExamplePage>
            </>
        }
    }
}
