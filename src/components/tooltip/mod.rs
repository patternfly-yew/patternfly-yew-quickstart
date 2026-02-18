use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TooltipExample)]
pub fn tooltip() -> Html {
    let example1 = example! ("Tooltip Popup" => "tooltip.1.example");
    let example2 = example! ("Tooltip" => "tooltip.2.example");

    html! {
        <>
            <ExamplePage title="Tooltip">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
