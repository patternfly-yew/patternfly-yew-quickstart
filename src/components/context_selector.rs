use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct ContextSelectorExample {}

impl Component for ContextSelectorExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let examples: Vec<Html> =
            vec![example2! ("Context Selector" => "context_selector.1.example")];

        html! {
            <>
                <ExamplePage title="Context Selector">
                    { for examples }
                </ExamplePage>
            </>
        }
    }
}
