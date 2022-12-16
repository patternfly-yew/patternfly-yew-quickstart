use crate::{example::ExamplePage, example2};
use patternfly_yew::*;
use std::time::Duration;
use yew::prelude::*;

#[function_component(ToastExample)]
pub fn toast() -> Html {
    let toaster = use_toaster().expect("Must be nested inside a ToastViewer");

    let example1 = example2!("Toast" => "toast.1.example");

    html!(
        <ExamplePage title="Tooltip">
            {example1}
        </ExamplePage>
    )
}
