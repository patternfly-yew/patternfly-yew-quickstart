use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(MenuExample)]
pub fn menu_example() -> Html {
    let toaster = use_toaster().unwrap();
    let action = use_callback(toaster, |_, toaster| toaster.toast("Clicked it!"));

    let example1 = example!("Menu Toggle" => "menu.1.example");
    let example2 = example!("Menu" => "menu.2.example");
    let example3 = example!("Full Menu" => "menu.3.example");
    let example4 = example!("Simple Drop Down" => "menu.4.example");

    html! {
        <ExamplePage title="Menu">{ example1 }{ example2 }{ example3 }{ example4 }</ExamplePage>
    }
}
