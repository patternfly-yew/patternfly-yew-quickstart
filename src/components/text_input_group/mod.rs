use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(TextInputGroupExample)]
pub fn text_input_group() -> Html {
    let example1 = example! ("Basic" => "text_input_group.1.example");
    let example2 = example! ("Disabled" => "text_input_group.2.example");

    html! {
        <>
            <ExamplePage title="TextInputGroup">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
