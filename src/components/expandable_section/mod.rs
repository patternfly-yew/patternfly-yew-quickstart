use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ExpandableSectionExample)]
pub fn example() -> Html {
    let example1 = example! ("Expandable Section" => "expandable_section.1.example");
    let example2 = example! ("Expandable Section (Indented)" => "expandable_section.2.example");
    let example3 =
        example! ("Expandable Section (Disclosure variation)" => "expandable_section.3.example");
    let example4 = example! ("Expandable Section (Detached)" => "expandable_section.4.example");

    html! (
        <>
            <ExamplePage title="Expandable Section">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
            </ExamplePage>
        </>
    )
}
