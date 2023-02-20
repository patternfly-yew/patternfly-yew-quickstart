use crate::{example::ExamplePage, example2};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(CodeBlockExample)]
pub fn example() -> Html {
    let example1 = example2! ("Code Block" => "code_block.1.example");
    let example2 = example2! ("Code Block (Expandable)" => "code_block.2.example");

    html! (
        <>
            <ExamplePage title="Code Block">
                {example1}
                {example2}
            </ExamplePage>
        </>
    )
}
