use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BullseyeExample)]
pub fn bullseye_example() -> Html {
    let example1 = example! ("Bullseye" => "bullseye.1.example");

    html! {
        <>
            <ExamplePage title="Bullseye Layout">{ example1 }</ExamplePage>
        </>
    }
}
