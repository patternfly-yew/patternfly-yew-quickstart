use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BackToTopExample)]
pub fn back_to_top_example() -> Html {
    let example1 = example!("Always visible" => "back_to_top.1.example");
    let example2 = example!("Scroll to reveal (default)" => "back_to_top.2.example");
    let example3 = example!("Custom title" => "back_to_top.3.example");
    html! {
        <ExamplePage title="Back to Top">
            {example1}
            {example2}
            {example3}
        </ExamplePage>
    }
}
