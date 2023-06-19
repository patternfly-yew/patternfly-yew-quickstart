use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(HelperTextExample)]
pub fn helper_text() -> Html {
    let example1 = example! ("HelperText" => "helper_text.1.example");
    let example2 = example! ("HelperText (With custom icon)" => "helper_text.2.example");

    html! {
        <>
            <ExamplePage title="DescriptionList">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
