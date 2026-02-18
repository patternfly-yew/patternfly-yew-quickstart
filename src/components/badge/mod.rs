use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BadgeExample)]
pub fn badge_example() -> Html {
    let example1 = example! ("Read" => "badge.1.example");
    let example2 = example! ("Unread with Screen Reader Text" => "badge.2.example");

    html! {
        <>
            <ExamplePage title="Badge">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
