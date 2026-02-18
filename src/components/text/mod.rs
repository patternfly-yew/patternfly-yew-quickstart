use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TextExample)]
pub fn text_example() -> Html {
    let example1 = example! ("Headings" => "text.1.example");
    let example2 = example! ("Headings" => "text.2.example");

    html! {
        <>
            <ExamplePage title="Text">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
