use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SelectExample)]
pub fn select_example() -> Html {
    let example1 = example2! ("Select" => "select.1.example");
    let example2 = example2! ("Select (with divider)" => "select.2.example");
    let example3 = example2! ("Select (with groups)" => "select.3.example");
    let example4 = example2! ("Select Checkbox (with groups)" => "select.4.example");

    html! {
        <>
            <ExamplePage title="Select">
                {example1}
                {example2}
                {example3}
                {example4}
            </ExamplePage>
        </>
    }
}
