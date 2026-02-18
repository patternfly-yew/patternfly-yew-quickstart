use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ToggleGroupExample)]
pub fn toast() -> Html {
    let example1 = example!("Basic" => "toggle_group.1.example");
    let example2 = example!("Single Select" => "toggle_group.2.example");
    let example3 = example!("Icons" => "toggle_group.3.example");
    let example4 = example!("Icons and Text" => "toggle_group.4.example");
    let example5 = example!("Compact" => "toggle_group.5.example");

    html!(
        <ExamplePage title="Toggle Group">
            { example1 }
            { example2 }
            { example3 }
            { example4 }
            { example5 }
        </ExamplePage>
    )
}
