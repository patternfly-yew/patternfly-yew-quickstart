use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(ModalExample)]
pub fn modal_example() -> Html {
    let example1 = example2!("Basic Medium Modal" => "modal.1.example");
    let example2 = example2!("Modal with a form" => "modal.2.example");

    html! {
        <>
            <ExamplePage title="Modal">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
