use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(TitleExample)]
pub fn title_example() -> Html {
    let example1 = example2! ("Headings" => "title.1.example");
    let example2 = example2! ("Headings" => "title.2.example");

    html! {
        <>
            <ExamplePage title="Title">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
