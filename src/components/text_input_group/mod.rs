use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TextInputGroupExample)]
pub fn text_input_group() -> Html {
    let example1 = example! ("Basic" => "text_input_group.1.example");
    let example2 = example! ("Disabled" => "text_input_group.2.example");
    let example3 =
        example! ("Utilities and icon with placeholder text" => "text_input_group.3.example");

    html! {
        <>
            <ExamplePage title="Text Input Group">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
