use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AlertExample)]
pub fn alert() -> Html {
    let example1 = example! ("Alerts" => "alert.1.example");
    let example2 = example! ("Alerts (inline)" => "alert.2.example");

    html! {
        <>
            <ExamplePage title="Alerts">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
