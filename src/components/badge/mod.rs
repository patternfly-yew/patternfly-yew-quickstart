use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(BadgeExample)]
pub fn badge_example() -> Html {
    let example1 = example! ("Badge" => "badge.1.example");
    let example2 = example! ("Badge (Read-only)" => "badge.2.example");

    html! {
        <>
            <ExamplePage title="Badge">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
