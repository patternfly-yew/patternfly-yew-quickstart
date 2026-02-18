use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ProgressExample)]
pub fn progress() -> Html {
    let example1 = example! ("Progress" => "progress.1.example");
    let example2 = example! ("Progress (variants)" => "progress.2.example");
    let example3 = example! ("Progress (helper text)" => "progress.3.example");
    let example4 = example! ("Progress (single line)" => "progress.4.example");
    let example5 = example! ("Progress (value format)" => "progress.5.example");

    html! {
        <>
            <ExamplePage title="Progress">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
            </ExamplePage>
        </>
    }
}
