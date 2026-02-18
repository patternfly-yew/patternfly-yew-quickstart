use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SkeletonExample)]
pub fn skeleton() -> Html {
    let example1 = example! ("Default" => "skeleton.1.example");
    let example2 = example! ("Percentage widths" => "skeleton.2.example");
    let example3 = example! ("Percentage heights" => "skeleton.3.example");
    let example4 = example! ("Text modifiers" => "skeleton.4.example");
    let example5 = example! ("Shapes" => "skeleton.5.example");

    html! {
        <>
            <ExamplePage title="Skeleton">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
            </ExamplePage>
        </>
    }
}
