use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(HintExample)]
pub fn hint_example() -> yew::Html {
    let example1 = example! ("Hint with Title, Footer and Actions" => "hint.1.example");
    let example2 = example! ("Minimal Hint" => "hint.2.example");
    let example3 = example! ("Hint with Footer" => "hint.3.example");

    html! {
        <>
            <ExamplePage title="Hints">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
