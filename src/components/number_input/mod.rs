use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(NumberExample)]
pub fn modal_example() -> Html {
    let example1 = example!("Basic number input" => "number_input.1.example");
    let example2 = example!("With unit" => "number_input.2.example");
    let example3 = example!("With thresholds" => "number_input.3.example");
    let example4 = example!("Disabled" => "number_input.4.example");
    let example5 = example!("With status" => "number_input.5.example");
    let example6 = example!("With custom width" => "number_input.6.example");

    html! {
        <>
            <ExamplePage title="Number input">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
                { example6 }
            </ExamplePage>
        </>
    }
}
