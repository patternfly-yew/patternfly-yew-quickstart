use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(HeroExample)]
pub fn hero_example() -> Html {
    let example1 = example! ("Basic" => "hero.1.example");
    let example2 = example! ("With gradient" => "hero.2.example");
    let example3 = example! ("With background image" => "hero.3.example");
    let example4 = example! ("Custom body width" => "hero.4.example");

    html! {
        <ExamplePage title="Hero">
            { example1 }{ example2 }{ example3 }{ example4 }
        </ExamplePage>
    }
}
