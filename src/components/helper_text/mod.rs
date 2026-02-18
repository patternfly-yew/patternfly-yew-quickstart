use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(HelperTextExample)]
pub fn helper_text() -> Html {
    let example1 = example! ("HelperText" => "helper_text.1.example");
    let example2 = example! ("HelperText (using an ul component)" => "helper_text.2.example");
    let example3 = example! ("HelperText (icon property options)" => "helper_text.3.example");

    html! {
        <>
            <ExamplePage title="HelperText">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
