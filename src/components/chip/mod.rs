use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ChipExample)]
pub fn chip() -> Html {
    let example1 = example! ("Chip" => "chip.1.example");
    let example2 = example! ("Chip (with badge)" => "chip.2.example");
    let example3 = example! ("Chip (draggable)" => "chip.3.example");
    let example4 = example! ("Chip (onclose)" => "chip.4.example");
    let example5 = example! ("Chip Groups" => "chip.5.example");

    html! {
        <>
            <ExamplePage title="Chip">
                {example1}
                {example2}
                {example3}
                {example4}
                {example5}
            </ExamplePage>
        </>
    }
}
