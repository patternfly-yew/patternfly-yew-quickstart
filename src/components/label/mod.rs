use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[component(LabelExample)]
pub fn label_example() -> Html {
    let example1 = example!("Label" => "label.1.example");
    let example2 = example!("Label (outline)" => "label.2.example");

    html! (
        <>
            <ExamplePage title="Label">{ example1 }{ example2 }</ExamplePage>
        </>
    )
}
