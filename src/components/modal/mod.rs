use crate::{example, example::ExamplePage};
use patternfly_yew::next::TextInput;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ModalExample)]
pub fn modal_example() -> Html {
    let example1 = example!("Basic Medium Modal" => "modal.1.example");
    let example2 = example!("Modal with a form" => "modal.2.example");

    html! {
        <>
            <ExamplePage title="Modal">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
