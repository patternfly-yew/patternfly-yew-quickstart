use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(ContextSelectorExample)]
pub fn example() -> Html {
    let examples: Vec<Html> = vec![
        example! ("Context Selector" => "context_selector.1.example"),
        example! ("Context Selector (form)" => "context_selector.2.example"),
    ];

    html! {
        <>
            <ExamplePage title="Context Selector">
                { for examples }
            </ExamplePage>
        </>
    }
}
