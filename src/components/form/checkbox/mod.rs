use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(FormCheckboxExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic Checkbox" => "checkbox.1.example" };
    let example2 = example! {"Tri-state Checkbox" => "checkbox.2.example" };

    html! (<ExamplePage title="Checkbox">{ example1 }{ example2 }</ExamplePage>)
}
