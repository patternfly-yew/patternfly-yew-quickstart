use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(MenuExample)]
pub fn menu_example() -> Html {
    let example1 = example!("Basic Menu" => "menu.1.example");

    html! {
        <>
            <ExamplePage title="Menu">
                {example1}
            </ExamplePage>
        </>
    }
}
