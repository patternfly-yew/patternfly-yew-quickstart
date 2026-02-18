use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AccordionExample)]
pub fn accordion() -> Html {
    let example1 = example! ("Accordion" => "accordion.1.example");
    let example2 = example! ("Accordion (multi)" => "accordion.2.example");
    let example3 = example! ("Accordion (modifiers)" => "accordion.3.example");

    html! {
        <>
            <ExamplePage title="Accordion">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
