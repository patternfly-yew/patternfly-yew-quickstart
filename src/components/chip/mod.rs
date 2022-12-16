use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(ChipExample)]
pub fn chip() -> Html {
    let example1 = example2! ("Chip" => "chip.1.example");
    let example2 = example2! ("Chip (with badge)" => "chip.2.example");
    let example3 = example2! ("Chip (draggable)" => "chip.3.example");
    let example4 = example2! ("Chip (onclose)" => "chip.4.example");

    html! {
        <>
            <Chip/>
            <ExamplePage title="Chip">
                {example1}
                {example2}
                {example3}
                {example4}
            </ExamplePage>
        </>
    }
}
