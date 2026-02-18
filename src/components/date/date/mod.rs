use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(DatePickerExample)]
pub fn datepicker_example() -> Html {
    let date = example! ("Basic" => "date.1.example");

    html! {
        <>
            <ExamplePage title="Date Picker">{ date }</ExamplePage>
        </>
    }
}
