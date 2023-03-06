use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SliderExample)]
pub fn slider() -> Html {
    let example1 = example! ("Slider" => "slider.1.example");
    let example2 = example! ("Slider (Labels)" => "slider.2.example");
    let example3 = example! ("Slider (negative)" => "slider.3.example");

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
