mod colors;

use crate::{example, example::ExamplePage};
use colors::COLORS;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SearchInputExample)]
pub fn search_input() -> Html {
    let example1 = example! ("Basic" => "search_input.1.example");
    let example2 = example! ("Match with result count" => "search_input.2.example");
    let example3 = example! ("Match with navigable options" => "search_input.3.example");
    let example4 = example! ("With submit button" => "search_input.4.example");
    let example5 = example! ("With expandable button" => "search_input.5.example");
    let example6 = example! ("Auto-complete" => "search_input.6.example");

    html! {
        <>
            <ExamplePage title="Search Input">
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
