use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TimestampExample)]
pub fn timestamp_example() -> Html {
    let example1 = example! ("Default" => "timestamp.1.example");
    let example2 = example! ("Basic formats" => "timestamp.2.example");
    let example3 = example! ("Custom format" => "timestamp.3.example");
    let example4 = example! ("Default tooltip" => "timestamp.4.example");
    let example5 = example! ("Custom content" => "timestamp.5.example");
    let example6 = example! ("Custom tooltip" => "timestamp.6.example");

    html! {
        <ExamplePage title="Timestamp">
            { example1 }
            { example2 }
            { example3 }
            { example4 }
            { example5 }
            { example6 }
        </ExamplePage>
    }
}
