use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TitleExample)]
pub fn title_example() -> Html {
    let example1 = example! ("Headings" => "title.1.example");
    let example2 = example! ("Headings" => "title.2.example");

    html! {
        <>
            <ExamplePage title="Title">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
