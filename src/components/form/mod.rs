use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(FormExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic Form" => "form.1.example" };
    let example2 = example! {"Form States" => "form.2.example" };
    let example3 = example! {"Validation" => "form.3.example" };

    html! (
        <ExamplePage title="Form">
            { example1 }
            { example2 }
            { example3 }
        </ExamplePage>
    )
}
