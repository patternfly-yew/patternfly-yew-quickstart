use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(CardExample)]
pub fn card() -> Html {
    let example1 = example! ("Basic" => "card.1.example");
    let example2 = example! ("Modifier" => "card.2.example");
    let example3 = example! ("Header images and actions" => "card.3.example");
    let example4 = example! ("Title inline with images and actions" => "card.4.example");
    let example5 = example! ("Card header without title" => "card.5.example");
    let example14 = example! ("With HTML heading element" => "card.14.example");
    let example6 = example! ("With multiple body sections" => "card.6.example");
    let example7 = example! ("With a primary body section that fills" => "card.7.example");
    let example8 = example! ("Selectable" => "card.8.example");
    let example9 = example! ("Single selectable" => "card.9.example");
    let example10 = example! ("Clickable" => "card.10.example");
    let example11 = example! ("Clickable and selectable" => "card.11.example");
    let example12 = example! ("Expandable" => "card.12.example");
    let example13 = example! ("Expandable with icon" => "card.13.example");

    {
        html! {
            <>
                <ExamplePage title="Card">
                    { example1 }
                    { example2 }
                    { example3 }
                    { example4 }
                    { example5 }
                    { example14 }
                    { example6 }
                    { example7 }
                    { example8 }
                    { example9 }
                    { example10 }
                    { example11 }
                    { example12 }
                    { example13 }
                </ExamplePage>
            </>
        }
    }
}
