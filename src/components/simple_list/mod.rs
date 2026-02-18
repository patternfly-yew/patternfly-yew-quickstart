use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SimpleListExample)]
pub fn search_input() -> Html {
    let example1 = example! ("Basic" => "simple_list.1.example");
    let example2 = example! ("Grouped list" => "simple_list.2.example");
    // let example3 = example! ("Match with navigable options" => "search_input.3.example");
    // let example4 = example! ("With submit button" => "search_input.4.example");
    // let example5 = example! ("With expandable button" => "search_input.5.example");
    // let example6 = example! ("Auto-complete" => "search_input.6.example");

    html! {
        <>
            <ExamplePage title="Simple List">
                { example1 }
                { example2 }
                // {example3}
                // {example4}
                // {example5}
                // {example6}
            </ExamplePage>
        </>
    }
}
