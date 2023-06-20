use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SelectExample)]
pub fn select_example() -> Html {
    let example6_state = use_state(|| "".to_string());

    let example1 = example! ("Select" => "select.1.example");
    let example2 = example! ("Select (with divider)" => "select.2.example");
    let example3 = example! ("Select (with groups)" => "select.3.example");
    let example4 = example! ("Select Checkbox (with groups)" => "select.4.example");
    let example5 = example! ("Select (multiple)" => "select.5.example");
    let example6 = example! ("Select (multiple) with callback" => "select.6.example");

    html! {
        <>
            <ExamplePage title="Select">
                {example1}
                {example2}
                {example3}
                {example4}
                {example5}
                {example6}
            </ExamplePage>
        </>
    }
}
