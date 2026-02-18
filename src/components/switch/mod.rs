use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SwitchExample)]
pub fn switch_example() -> Html {
    let example1 = example! ("Switch" => "switch.1.example");
    let example2 = example! ("Switch (label)" => "switch.2.example");
    let example3 = example! ("Switch (disabled)" => "switch.3.example");

    html! {
        <>
            <ExamplePage title="Switch">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
