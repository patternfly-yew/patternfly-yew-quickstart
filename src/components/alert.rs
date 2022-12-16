use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(AlertExample)]
pub fn alert() -> Html {
    let example1 = example2! ("Alerts" => "alert.1.example");
    let example2 = example2! ("Alerts (inline)" => "alert.2.example");

    html! {
        <>
            <ExamplePage title="Alerts">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
