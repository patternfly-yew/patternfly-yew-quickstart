use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ListExample)]
pub fn list() -> Html {
    let example1 = example! ("Basic" => "list.1.example");
    let example2 = example! ("Inline" => "list.2.example");
    let example3 = example! ("Ordered" => "list.3.example");
    let example4 = example! ("Plain" => "list.4.example");
    let example5 = example! ("Bordered" => "list.5.example");

    html! {
        <>
            <ExamplePage title="List">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
            </ExamplePage>
        </>
    }
}
