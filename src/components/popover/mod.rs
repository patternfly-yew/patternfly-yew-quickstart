use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(PopoverExample)]
pub fn popover_example() -> Html {
    let example1 = example! ("Popover Popup" => "popover.1.example");
    let example2 = example! ("Popover" => "popover.2.example");

    html! {
        <>
            <ExamplePage title="Popover">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
