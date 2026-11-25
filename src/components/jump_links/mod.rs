use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(JumpLinksExample)]
pub fn jump_links_example() -> Html {
    let example1 = example!("Basic" => "jump_links.1.example");
    let example2 = example!("With label" => "jump_links.2.example");
    let example3 = example!("Horizontal" => "jump_links.3.example");
    let example4 = example!("With active item" => "jump_links.4.example");
    html! {
        <ExamplePage title="Jump Links">
            {example1}
            {example2}
            {example3}
            {example4}
        </ExamplePage>
    }
}
