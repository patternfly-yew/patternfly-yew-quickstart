use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(MasonryExample)]
pub fn masonry_example() -> Html {
    let example1 = example!("Masonry" => "masonry.1.example");
    let example2 = example!("Masonry (gutter)" => "masonry.2.example");
    let example3 = example!("Masonry (responsive columns)" => "masonry.3.example");
    html! {
        <ExamplePage title="Masonry Layout">
            {example1}
            {example2}
            {example3}
        </ExamplePage>
    }
}
