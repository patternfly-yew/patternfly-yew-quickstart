use crate::{example, example::ExamplePage};
use chrono::*;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(CalendarExample)]
pub fn datepicker_example() -> yew::Html {
    let date = use_state(|| NaiveDate::from_ymd_opt(2011, 1, 14).unwrap());
    let onchange = {
        let date = date.clone();
        Callback::from(move |d| date.set(d))
    };

    let select_date = example! ("Selectable date" => "calendar.1.example");
    let range = example! ("Date range" => "calendar.2.example");

    html! {
        <>
            <ExamplePage title="Calendar month">{ select_date }{ range }</ExamplePage>
        </>
    }
}
