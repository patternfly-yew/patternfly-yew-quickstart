use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SliderExample)]
pub fn slider() -> Html {
    let msg = use_state(|| String::new());

    let example1 = example2! ("Slider" => "slider.1.example");
    let example2 = example2! ("Slider (Labels)" => "slider.2.example");
    let example3 = example2! ("Slider (negative)" => "slider.3.example");

    html! (
        <>
            <ExamplePage title="Sliders">
                {example1}
                {example2}
                {example3}
            </ExamplePage>
        </>
    )
}
