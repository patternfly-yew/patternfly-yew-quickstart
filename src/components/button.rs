use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(ButtonExample)]
pub fn button_example() -> yew::Html {
    let variants = example2! ("Variants" => "button.1.example");
    let variants_disabled = example2! ("Variants (disabled)" => "button.2.example");
    let progress = example2! ("Progress indicator" => "button.3.example");
    let icons = example2! ("With Icons" => "button.4.example");

    html! {
        <>
            <ExamplePage title="Buttons">
                { variants }
                { variants_disabled }
                { progress }
                { icons }
            </ExamplePage>
        </>
    }
}
