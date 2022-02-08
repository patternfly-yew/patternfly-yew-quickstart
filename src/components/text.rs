use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(TextExample)]
pub fn text_example() -> Html {
    let example1 = example2! ("Headings" => "text.1.example");
    let example2 = example2! ("Headings" => "text.2.example");

    html! {
        <>
            <ExamplePage title="Text">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
