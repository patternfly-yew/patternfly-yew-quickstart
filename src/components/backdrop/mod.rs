use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BackdropExample)]
pub fn backdrop() -> Html {
    let backdropper = use_backdrop().expect("Requires BackdropViewer in its hierarchy");

    let example1 = example! ("Backdrop" => "backdrop.1.example");

    html! {
        <>
            <ExamplePage title="Backdrop">{ example1 }</ExamplePage>
        </>
    }
}
