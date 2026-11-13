use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(LevelExample)]
pub fn level_example() -> Html {
    let example1 = example!("Level" => "level.1.example");
    let example2 = example!("Level (gutter)" => "level.2.example");
    html! { <ExamplePage title="Level Layout">{ example1 }{ example2 }</ExamplePage> }
}
