use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct EmptyStateExample {}

impl Component for EmptyStateExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let example1 = example2! ("Empty state" => "empty.1.example");
        let example2 = example2! ("Empty state (XLarge)" => "empty.2.example");

        html! {
            <>
                <ExamplePage title="Empty state">
                    {example1}
                    {example2}
                </ExamplePage>
            </>
        }
    }
}
