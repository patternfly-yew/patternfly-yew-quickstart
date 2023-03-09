use crate::example;
use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TreeExample)]
pub fn tree_example() -> Html {
    let example1 = example! ("Basic" => "tree.1.example");

    html! (
        <>
            <ExamplePage title="Tree Table" experimental=true>
                {example1}
            </ExamplePage>
        </>
    )
}
