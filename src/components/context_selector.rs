use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct ContextSelectorExample {}

impl Component for ContextSelectorExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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
