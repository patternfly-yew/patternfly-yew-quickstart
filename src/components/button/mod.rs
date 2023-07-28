use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ButtonExample)]
pub fn button_example() -> yew::Html {
    let variants = example! ("Variants" => "button.1.example");
    let variants_disabled = example! ("Variants (disabled)" => "button.2.example");
    let progress = example! ("Progress indicator" => "button.3.example");
    let icons = example! ("With Icons" => "button.4.example");
    // let anchor = example! ("A element" => "button.5.example");

    html! {
        <>
            <ExamplePage title="Buttons">
                { variants }
                { variants_disabled }
                { progress }
                { icons }
                // { anchor }
            </ExamplePage>
        </>
    }
}
