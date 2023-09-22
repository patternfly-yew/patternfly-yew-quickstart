use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use chrono::*;

#[function_component(DatePickerExample)]
pub fn datepicker_example() -> yew::Html {
    let date = example! ("Basic" => "date.1.example");

    html! {
        <>
            <ExamplePage title="DatePicker">
                { date }
            </ExamplePage>
        </>
    }
}
