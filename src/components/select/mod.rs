use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SelectExample)]
pub fn select_example() -> Html {
    let example1 = example! ("Select" => "select.1.example");
    let example2 = example! ("Select (with divider)" => "select.2.example");
    let example3 = example! ("Select (with groups)" => "select.3.example");
    let example4 = example! ("Select Checkbox (with groups)" => "select.4.example");
    let example5 = example! ("Select (multiple)" => "select.5.example");

    html! {
        <>
            <ExamplePage title="Select">
                {example1}
                {example2}
                {example3}
                {example4}
                {example5}
            </ExamplePage>
        </>
    }
}
