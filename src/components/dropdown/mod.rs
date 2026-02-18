use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(DropdownExample)]
pub fn dropdown_example() -> Html {
    let toaster = use_toaster().unwrap();
    let show_toast = use_callback(toaster, |text, toaster| toaster.toast(text));

    let example1 = example! ("Dropdown" => "dropdown.1.example");
    let example2 = example! ("Dropdown (Kebab)" => "dropdown.2.example");
    let example3 = example! ("Dropdown (User)" => "dropdown.3.example");

    html! {
        <>
            <ExamplePage title="Dropdown">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
