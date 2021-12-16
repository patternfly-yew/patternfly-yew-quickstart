use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SwitchExample)]
pub fn switch_example() -> Html {
    let example1 = example2! ("Switch" => "switch.1.example");
    let example2 = example2! ("Switch (label)" => "switch.2.example");
    let example3 = example2! ("Switch (disabled)" => "switch.3.example");

    html! {
        <>
            <ExamplePage title="Switch">
                {example1}
                {example2}
                {example3}
            </ExamplePage>
        </>
    }
}
