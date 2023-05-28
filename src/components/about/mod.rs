use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(AboutExample)]
pub fn about_example() -> Html {

    let backdropper = use_backdrop().expect("Requires BackdropViewer in its hierarchy");

    let example1 = example! ("Basic" => "about.1.example");

    html!(
        <ExamplePage title="About">
            { example1 }
        </ExamplePage>
    )
}
