use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct EmptyStateExample {}

impl Component for EmptyStateExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let example1 = example! ("Empty state" => "empty.1.example");
        let example2 = example! ("Empty state (XLarge)" => "empty.2.example");

        html! {
            <>
                <ExamplePage title="Empty state">{ example1 }{ example2 }</ExamplePage>
            </>
        }
    }
}
