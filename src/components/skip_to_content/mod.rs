use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SkipToContentExample)]
pub fn skip_to_content_example() -> Html {
    let example1 = example!("Basic" => "skip_to_content.1.example");
    let example2 = example!("Custom text" => "skip_to_content.2.example");
    html! {
        <ExamplePage title="Skip to Content">
            {example1}
            {example2}
        </ExamplePage>
    }
}
