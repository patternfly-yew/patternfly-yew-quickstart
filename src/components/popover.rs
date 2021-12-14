use crate::{example::Example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(PopoverExample)]
pub fn popover_example() -> Html {
    html! {
        <>
            <ExamplePage title="Popover">
                <Example title="Popover Popup" code={include_str!("popover.1.example")}>{include!("popover.1.example")}</Example>
                <Example title="Popover" code={include_str!("popover.2.example")}>{include!("popover.2.example")}</Example>
            </ExamplePage>
        </>
    }
}
