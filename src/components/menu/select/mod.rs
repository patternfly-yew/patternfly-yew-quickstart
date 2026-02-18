use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SimpleSelectExample)]
pub fn select_example() -> Html {
    let example1 = example!("Simple Select" => "select.1.example");

    html! {
        <>
            <ExamplePage title="Select">{ example1 }</ExamplePage>
        </>
    }
}
