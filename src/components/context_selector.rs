use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(ContextSelectorExample)]
pub fn example() -> Html {
    let examples: Vec<Html> = vec![
        example2! ("Context Selector" => "context_selector.1.example"),
        example2! ("Context Selector (form)" => "context_selector.2.example"),
    ];

    html! {
        <>
            <ExamplePage title="Context Selector">
                { for examples }
            </ExamplePage>
        </>
    }
}
