use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(CardExample)]
pub fn card() -> Html {
    let compact = use_state(|| false);
    let flat = use_state(|| false);
    let full_height = use_state(|| false);
    let large = use_state(|| false);
    let plain = use_state(|| false);
    let rounded = use_state(|| false);
    let example1 = example! ("Basic" => "card.1.example");
    let example2 = example! ("Modifier" => "card.2.example");
    let example3 = example! ("Expandable" => "card.3.example");
    let example4 = example! ("Card Selection" => "card.4.example");

    {

        html! {
            <>
                <ExamplePage title="Card">
                    {example1}
                    {example2}
                    {example3}
                    {example4}
                </ExamplePage>
            </>
        }
    }
}
