use crate::{example, example::ExamplePage};
use patternfly_yew::next::TextInput;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(FormExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic Form" => "form.1.example" };
    let example2 = example! {"Text Input" => "form.2.example" };
    let example3 = example! {"Form States" => "form.3.example" };
    let example4 = example! {"Validation" => "form.4.example" };

    html! (
        <ExamplePage title="Form">
            { example1 }
            { example2 }
            { example3 }
            { example4 }
        </ExamplePage>
    )
}
